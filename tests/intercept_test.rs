#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use futures_core::future::BoxFuture;
    use log::{Log, Metadata, Record};
    use rbatis::executor::Executor;
    use rbatis::intercept::{Intercept, ResultType};
    use rbatis::{Error, RBatis};
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, MetaData, Row};
    use rbdc::rt::block_on;
    use rbs::Value;
    use std::sync::atomic::{AtomicI64, Ordering};
    use std::sync::Arc;

    pub struct Logger {}

    impl Log for Logger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            return true;
        }

        fn log(&self, record: &Record) {
            println!(
                "[{}]{}",
                record.module_path_static().unwrap(),
                record.args()
            )
        }

        fn flush(&self) {}
    }

    #[derive(Debug, Clone)]
    struct MockDriver {}

    impl Driver for MockDriver {
        fn name(&self) -> &str {
            "test"
        }

        fn connect(&self, _url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn connect_opt<'a>(
            &'a self,
            _opt: &'a dyn ConnectOptions,
        ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn default_option(&self) -> Box<dyn ConnectOptions> {
            Box::new(MockConnectOptions {})
        }
    }

    #[derive(Clone, Debug)]
    struct MockRowMetaData {
        sql: String,
    }

    impl MetaData for MockRowMetaData {
        fn column_len(&self) -> usize {
            if self.sql.contains("select count") {
                1
            } else {
                2
            }
        }

        fn column_name(&self, i: usize) -> String {
            if self.sql.contains("select count") {
                "count".to_string()
            } else {
                if i == 0 {
                    "sql".to_string()
                } else {
                    "count".to_string()
                }
            }
        }

        fn column_type(&self, _i: usize) -> String {
            "String".to_string()
        }
    }

    #[derive(Clone, Debug)]
    struct MockRow {
        pub sql: String,
        pub count: u64,
    }

    impl Row for MockRow {
        fn meta_data(&self) -> Box<dyn MetaData> {
            Box::new(MockRowMetaData {
                sql: self.sql.clone(),
            }) as Box<dyn MetaData>
        }

        fn get(&mut self, i: usize) -> Result<Value, Error> {
            if self.sql.contains("select count") {
                Ok(Value::U64(self.count))
            } else {
                if i == 0 {
                    Ok(Value::String(self.sql.clone()))
                } else {
                    Ok(Value::U64(self.count.clone()))
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnection {}

    impl Connection for MockConnection {
        fn get_rows(
            &mut self,
            sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
            let sql = sql.to_string();
            Box::pin(async move {
                let data = Box::new(MockRow { sql: sql, count: 1 }) as Box<dyn Row>;
                Ok(vec![data])
            })
        }

        fn exec(
            &mut self,
            _sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<Result<ExecResult, Error>> {
            Box::pin(async move {
                Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::Null,
                })
            })
        }

        fn close(&mut self) -> BoxFuture<Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }

        fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnectOptions {}

    impl ConnectOptions for MockConnectOptions {
        fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn set_uri(&mut self, _uri: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct MockIntercept {}

    #[async_trait]
    impl Intercept for MockIntercept {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
        ) -> Result<Option<bool>, Error> {
            match result {
                ResultType::Exec(v) => {
                    *v = Ok(ExecResult {
                        rows_affected: 1,
                        last_insert_id: Value::U64(1),
                    });
                    Ok(None)
                }
                ResultType::Query(_) => Ok(Some(true)),
            }
        }
    }

    #[test]
    fn test_mock_intercept() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();
        rb.intercepts.clear();
        rb.intercepts.push(Arc::new(MockIntercept {}));
        let f = async move {
            let r = rb.exec("select * from table", vec![]).await.unwrap();
            println!("r={}", r);
            assert_eq!(
                r,
                ExecResult {
                    rows_affected: 1,
                    last_insert_id: Value::U64(1),
                }
            );
        };
        block_on(f);
    }

    #[test]
    fn test_get_intercept_type() {
        #[derive(Debug)]
        pub struct MockIntercept {
            pub inner: AtomicI64,
        }

        #[async_trait]
        impl Intercept for MockIntercept {}
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();
        rb.intercepts.push(Arc::new(MockIntercept {
            inner: AtomicI64::new(0),
        }));
        let m = rb.get_intercept::<MockIntercept>();
        assert_eq!(m.is_some(), true);
        println!("{}", m.unwrap().name());
        let m = m.unwrap();
        m.inner.store(1, Ordering::SeqCst);
        assert_eq!(m.inner.load(Ordering::Relaxed), 1);
    }
}
