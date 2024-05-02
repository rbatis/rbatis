#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod test {
    use dark_std::sync::SyncVec;
    use futures_core::future::BoxFuture;
    use rbatis::executor::{Executor, RBatisConnExecutor};
    use rbatis::intercept::{Intercept, ResultType};
    use rbatis::plugin::PageRequest;
    use rbatis::{Error, RBatis};
    use rbdc::datetime::DateTime;
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, MetaData, Row};
    use rbdc::rt::block_on;
    use rbs::{from_value, to_value, Value};
    use std::any::Any;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct MockIntercept {
        pub sql_args: Arc<SyncVec<(String, Vec<Value>)>>,
    }

    impl MockIntercept {
        fn new(inner: Arc<SyncVec<(String, Vec<Value>)>>) -> Self {
            Self { sql_args: inner }
        }
    }

    #[async_trait]
    impl Intercept for MockIntercept {
        async fn before(
            &self,
            task_id: i64,
            rb: &dyn Executor,
            sql: &mut String,
            args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
        ) -> Result<Option<bool>, Error> {
            self.sql_args.push((sql.to_string(), args.clone()));
            Ok(Some(true))
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
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            #[html_sql(
                r#"<mapper>
            <select id="select_by_condition">
            select ${id},${id},#{id},#{id}
            </select>
            </mapper>"#
            )]
            pub async fn test_same_id(rb: &RBatis, id: &u64) -> Result<Value, Error> {
                impled!()
            }
            let r = test_same_id(&mut rb, &1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select 1,1,?,?");
            assert_eq!(args, vec![Value::U64(1), Value::U64(1)]);
        };
        block_on(f);
    }

    #[test]
    fn test_macro() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);

            htmlsql!(test_same_id(rb: &RBatis, id: &u64)  -> Result<Value, Error> => r#"<mapper>
            <select id="test_same_id">
            select ${id},${id},#{id},#{id}
            </select>
            </mapper>"#);

            let r = test_same_id(&mut rb, &1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select 1,1,?,?");
            assert_eq!(args, vec![Value::U64(1), Value::U64(1)]);
        };
        block_on(f);
    }

    #[test]
    fn test_test_if() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);

            htmlsql!(test_if(rb: &RBatis, id: &u64)  -> Result<Value, Error> => "tests/test.html");

            let r = test_if(&mut rb, &1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select * from table where id = 1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_test_null() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);

            htmlsql!(test_null(rb: &RBatis, id: Option<i32>)  -> Result<Value, Error> => "tests/test.html");

            let r = test_null(&mut rb, None).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select * from table where id = 1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_method_call() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);

            pub trait Methods {
                fn method(&self) -> Value;
            }
            impl Methods for Value {
                fn method(&self) -> Value {
                    Value::I32(1)
                }
            }
            htmlsql!(test_method_call(rb: &RBatis, id: Option<i32>)  -> Result<Value, Error> => "tests/test.html");

            let r = test_method_call(&mut rb, None).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "select * from table where id = 1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_binary() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_binary(rb: &RBatis, id: i32, b:bool)  -> Result<Value, Error> => "tests/test.html");

            let r = test_binary(&mut rb, 1, true).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql.replace("\r\n","").replace("\n",""), "2,0,1,1,0,1,1,true,false,true,false,true,false,0,true,true,2,0,1,1,0,false,false,true,false,true,false,true,false,0,true,true,2,0,1,1,0,1,1,true,false,true,false,true,false,0,true,true,2,0,1,1,0,1,1,true,false,true,false,true,false,0,true,true");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_unary() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_unary(rb: &RBatis, id: i32)  -> Result<Value, Error> => "tests/test.html");

            let r = test_unary(&mut rb, 1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "-1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_paren() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_paren(rb: &RBatis, id: i32)  -> Result<Value, Error> => "tests/test.html");

            let r = test_paren(&mut rb, 1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "-1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_field() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);

            htmlsql!(test_field(rb: &RBatis, t: MockTable)  -> Result<Value, Error> => "tests/test.html");

            let r = test_field(
                &mut rb,
                MockTable {
                    id: None,
                    name: Some("aaa".to_string()),
                    pc_link: None,
                    h5_link: None,
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: None,
                    remark: None,
                    create_time: None,
                    version: None,
                    delete_flag: None,
                    count: 0,
                },
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "aaa");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_reference() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_reference(rb: &RBatis, t: MockTable)  -> Result<Value, Error> => "tests/test.html");

            let r = test_reference(
                &mut rb,
                MockTable {
                    id: None,
                    name: Some("aaa".to_string()),
                    pc_link: None,
                    h5_link: None,
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: None,
                    remark: None,
                    create_time: None,
                    version: None,
                    delete_flag: None,
                    count: 0,
                },
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "aaa");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_index() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_index(rb: &RBatis, arr: Vec<i32>, map:HashMap<String,String>)  -> Result<Value, Error> => "tests/test.html");
            let mut m = HashMap::new();
            m.insert("0".to_string(), "1".to_string());
            let r = test_index(&rb, vec![1], m).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "11");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_lit() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_lit(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_lit(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "aaaa");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_where_empty() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_where_empty(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_where_empty(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_where_sql() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_where_sql(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_where_sql(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_bind() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_bind(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_bind(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("sql={},args={}", sql, Value::Array(args.clone()));
            assert_eq!(sql, "1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_choose() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_choose(rb: &RBatis,a:i32)  -> Result<Value, Error> => "tests/test.html");
            let r = test_choose(&rb, 1).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "true");
            assert_eq!(args, vec![]);
            let r = test_choose(&rb, 0).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "false");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_for() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_for(rb: &RBatis,ids:Vec<i32>)  -> Result<Value, Error> => "tests/test.html");
            let r = test_for(&rb, vec![0, 1, 2, 3]).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("sql={},args={}", sql, Value::Array(args.clone()));
            assert_eq!(sql, "(1,1),(2,2)");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_trim() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_trim(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_trim(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "()");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_trim2() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_trim2(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_trim2(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "()");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_trim3() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_trim3(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_trim3(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_trim4() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_trim3(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_trim3(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_include() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_include(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_include(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, "1");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }

    #[test]
    fn test_set() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            htmlsql!(test_set(rb: &RBatis)  -> Result<Value, Error> => "tests/test.html");
            let r = test_set(&rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(sql, " set 1 ");
            assert_eq!(args, vec![]);
        };
        block_on(f);
    }
}
