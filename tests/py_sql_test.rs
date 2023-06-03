#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(private_in_public)]

#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod test {
    #![allow(private_in_public)]

    use crossbeam::queue::SegQueue;
    use futures_core::future::BoxFuture;
    use rbatis::executor::{Executor, RBatisConnExecutor};
    use rbatis::intercept::{Either, SqlIntercept};
    use rbatis::sql::PageRequest;
    use rbatis::{Error, RBatis};
    use rbdc::datetime::DateTime;
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, MetaData, Row};
    use rbdc::rt::block_on;
    use rbs::{from_value, to_value, Value};
    use std::any::Any;
    use std::collections::HashMap;
    use std::sync::Arc;

    pub struct MockIntercept {
        pub sql_args: Arc<SegQueue<(String, Vec<Value>)>>,
    }

    impl MockIntercept {
        fn new(inner: Arc<SegQueue<(String, Vec<Value>)>>) -> Self {
            Self { sql_args: inner }
        }
    }

    impl SqlIntercept for MockIntercept {
        fn before(
            &self,
            task_id: i64,
            rb: &dyn Executor,
            sql: &mut String,
            args: &mut Vec<Value>,
        ) -> Result<(), Error> {
            self.sql_args.push((sql.to_string(), args.clone()));
            Ok(())
        }
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
            params: Vec<Value>,
        ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
            let sql = sql.to_string();
            Box::pin(async move {
                let data = Box::new(MockRow { sql: sql, count: 1 }) as Box<dyn Row>;
                Ok(vec![data])
            })
        }

        fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
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

        fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
            self
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    struct MockTable {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<rbdc::datetime::DateTime>,
        pub version: Option<i64>,
        pub delete_flag: Option<i32>,
        //exec sql
        pub count: u64, //page count num
    }

    #[test]
    fn test_query_decode() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SegQueue::new());
            rb.set_sql_intercepts(vec![Box::new(MockIntercept::new(queue.clone()))]);
            #[py_sql("select ${id},${id},#{id},#{id} ")]
            pub async fn test_same_id(rb: &mut RBatis, id: &u64) -> Result<Value, Error> {
                impled!()
            }
            let r = test_same_id(&mut rb, &1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select 1,1,?,?");
            assert_eq!(args, vec![Value::U64(1), Value::U64(1)]);
        };
        block_on(f);
    }
}
