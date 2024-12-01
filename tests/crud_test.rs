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
    use rbatis::intercept_page::PageIntercept;
    use rbatis::plugin::PageRequest;
    use rbatis::{impl_delete, impl_select, impl_select_page, impl_update};
    use rbatis::{DefaultPool, Error, RBatis};
    use rbdc::datetime::DateTime;
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, MetaData, Row};
    use rbdc::pool::conn_manager::ConnManager;
    use rbdc::pool::Pool;
    use rbdc::rt::block_on;
    use rbs::{from_value, to_value, Value};
    use std::any::Any;
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::str::FromStr;
    use std::sync::atomic::{AtomicI32, Ordering};
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
            _option: &'a dyn ConnectOptions,
        ) -> BoxFuture<'a, Result<Box<dyn Connection>, Error>> {
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

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
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
        pub create_time: Option<DateTime>,
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
            let r: Vec<MockTable> = rb
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
            let mut conn = rb.acquire().await.unwrap();
            let r: Vec<MockTable> = conn
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_query_decode_tx() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let mut tx = rb.acquire_begin().await.unwrap();
            let r: Vec<MockTable> = tx
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_query_decode_tx_commit_done() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let mut tx = rb.acquire_begin().await.unwrap();
            let r: Vec<MockTable> = tx
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
            assert_eq!(tx.done, false);
            tx.commit().await;
            assert_eq!(tx.done, true);
        };
        block_on(f);
    }

    #[test]
    fn test_query_decode_tx_rollback_done() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let mut tx = rb.acquire_begin().await.unwrap();
            let r: Vec<MockTable> = tx
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
            assert_eq!(tx.done, false);
            tx.rollback().await;
            assert_eq!(tx.done, true);
        };
        block_on(f);
    }

    #[test]
    fn test_query_decode_tx_guard() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let mut tx = rb.acquire_begin().await.unwrap().defer_async(|tx| async {});
            let r: Vec<MockTable> = tx
                .query_decode("select * from mock_table", vec![])
                .await
                .unwrap();
        };
        block_on(f);
    }

    crud!(MockTable {});

    #[test]
    fn test_link() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.link(MockDriver {}, "test").await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_init_option() {
        let f = async move {
            let mut rb = RBatis::new();
            let mut opts = MockConnectOptions {};
            opts.set_uri("test");
            rb.init_option::<MockDriver, MockConnectOptions, DefaultPool>(MockDriver {}, opts)
                .unwrap();
            rb.acquire().await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_init_pool() {
        let f = async move {
            let mut rb = RBatis::new();
            let mut opts = MockConnectOptions {};
            opts.set_uri("test");
            let pool = DefaultPool::new(ConnManager::new_opt_box(
                Box::new(MockDriver {}),
                Box::new(opts),
            ))
            .unwrap();
            rb.init_pool(pool).unwrap();
            rb.acquire().await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_try_acq() {
        let f = async move {
            let mut rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            rb.try_acquire().await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_insert() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let r = MockTable::insert(&mut rb, &t).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{} [{}]", sql, Value::from(args.clone()));
            assert_eq!(sql, "insert into mock_table (id,name,pc_link,h5_link,status,remark,create_time,version,delete_flag,count) VALUES (?,?,?,?,?,?,?,?,?,?)");
            assert_eq!(
                args,
                vec![
                    to_value!(t.id),
                    to_value!(t.name),
                    to_value!(t.pc_link),
                    to_value!(t.h5_link),
                    to_value!(t.status),
                    to_value!(t.remark),
                    to_value!(t.create_time),
                    to_value!(t.version),
                    to_value!(t.delete_flag),
                    to_value!(t.count),
                ]
            );
        };
        block_on(f);
    }

    #[test]
    fn test_insert_batch() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let mut t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("remark".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let mut t2 = t.clone();
            t.remark = None;
            t2.id = "3".to_string().into();
            let ts = vec![t, t2];
            let r = MockTable::insert_batch(&mut rb, &ts, 10).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{} [{}]", sql, Value::from(args.clone()));
            assert_eq!(sql, "insert into mock_table (id,name,pc_link,h5_link,status,remark,create_time,version,delete_flag,count) VALUES (?,?,?,?,?,?,?,?,?,?),(?,?,?,?,?,?,?,?,?,?)");
            assert_eq!(
                args,
                vec![
                    to_value!(&ts[0].id),
                    to_value!(&ts[0].name),
                    to_value!(&ts[0].pc_link),
                    to_value!(&ts[0].h5_link),
                    to_value!(&ts[0].status),
                    to_value!(&ts[0].remark),
                    to_value!(&ts[0].create_time),
                    to_value!(&ts[0].version),
                    to_value!(&ts[0].delete_flag),
                    to_value!(&ts[0].count),
                    to_value!(&ts[1].id),
                    to_value!(&ts[1].name),
                    to_value!(&ts[1].pc_link),
                    to_value!(&ts[1].h5_link),
                    to_value!(&ts[1].status),
                    to_value!(&ts[1].remark),
                    to_value!(&ts[1].create_time),
                    to_value!(&ts[1].version),
                    to_value!(&ts[1].delete_flag),
                    to_value!(&ts[1].count),
                ]
            );
        };
        block_on(f);
    }

    #[test]
    fn test_update_by_column() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let r = MockTable::update_by_column(&mut rb, &t, "id")
                .await
                .unwrap();

            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "update mock_table set name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=?,count=? where id = ?");
            assert_eq!(args.len(), 10);
            assert_eq!(
                args,
                vec![
                    to_value!(t.name),
                    to_value!(t.pc_link),
                    to_value!(t.h5_link),
                    to_value!(t.status),
                    to_value!(t.remark),
                    to_value!(t.create_time),
                    to_value!(t.version),
                    to_value!(t.delete_flag),
                    to_value!(t.count),
                    to_value!(t.id),
                ]
            );
        };
        block_on(f);
    }

    #[test]
    fn test_update_by_column_skip_null() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let r = MockTable::update_by_column_skip(&mut rb, &t, "id", false)
                .await
                .unwrap();

            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "update mock_table set name=?,pc_link=?,h5_link=?,pc_banner_img=?,h5_banner_img=?,sort=?,status=?,remark=?,create_time=?,version=?,delete_flag=?,count=? where id = ?");
            assert_eq!(args.len(), 13);
            assert_eq!(
                args,
                vec![
                    to_value!(t.name),
                    to_value!(t.pc_link),
                    to_value!(t.h5_link),
                    to_value!(t.pc_banner_img),
                    to_value!(t.h5_banner_img),
                    to_value!(t.sort),
                    to_value!(t.status),
                    to_value!(t.remark),
                    to_value!(t.create_time),
                    to_value!(t.version),
                    to_value!(t.delete_flag),
                    to_value!(t.count),
                    to_value!(t.id),
                ]
            );
        };
        block_on(f);
    }

    #[test]
    fn test_update_by_column_batch() {
        #[derive(Debug)]
        pub struct TestIntercept {
            pub num: AtomicI32,
        }

        #[async_trait]
        impl Intercept for TestIntercept {
            async fn before(
                &self,
                _task_id: i64,
                _rb: &dyn Executor,
                sql: &mut String,
                args: &mut Vec<Value>,
                _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
            ) -> Result<Option<bool>, Error> {
                assert_eq!(sql, "update mock_table set name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=?,count=? where id = ?");
                let num = self.num.load(Ordering::Relaxed) + 1;
                println!("{}", sql);
                println!("{}", Value::Array(args.clone()));
                assert_eq!(
                    args,
                    &[
                        Value::String(num.to_string()),
                        Value::String(num.to_string()),
                        Value::String(num.to_string()),
                        Value::I32(num),
                        Value::String(num.to_string()),
                        Value::Ext(
                            "DateTime",
                            Box::new(Value::String("2023-10-10T00:00:00+08:00".to_string()))
                        ),
                        Value::I64(num as i64),
                        Value::I32(num),
                        Value::U64(num as u64),
                        Value::String(num.to_string())
                    ]
                );
                self.num.fetch_add(1, Ordering::Relaxed);
                return Ok(Some(true));
            }
        }
        let f = async move {
            let mut rb = RBatis::new();
            rb.set_intercepts(vec![Arc::new(TestIntercept {
                num: Default::default(),
            })]);
            rb.init(MockDriver {}, "test").unwrap();
            let tables = vec![
                MockTable {
                    id: Some("1".into()),
                    name: Some("1".into()),
                    pc_link: Some("1".into()),
                    h5_link: Some("1".into()),
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: Some(1),
                    remark: Some("1".into()),
                    create_time: Some(DateTime::from_str("2023-10-10 00:00:00+08:00").unwrap()),
                    version: Some(1),
                    delete_flag: Some(1),
                    count: 1,
                },
                MockTable {
                    id: Some("2".into()),
                    name: Some("2".into()),
                    pc_link: Some("2".into()),
                    h5_link: Some("2".into()),
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: Some(2),
                    remark: Some("2".into()),
                    create_time: Some(DateTime::from_str("2023-10-10 00:00:00+08:00").unwrap()),
                    version: Some(2),
                    delete_flag: Some(2),
                    count: 2,
                },
                MockTable {
                    id: Some("3".into()),
                    name: Some("3".into()),
                    pc_link: Some("3".into()),
                    h5_link: Some("3".into()),
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: Some(3),
                    remark: Some("3".into()),
                    create_time: Some(DateTime::from_str("2023-10-10 00:00:00+08:00").unwrap()),
                    version: Some(3),
                    delete_flag: Some(3),
                    count: 3,
                },
                MockTable {
                    id: Some("4".into()),
                    name: Some("4".into()),
                    pc_link: Some("4".into()),
                    h5_link: Some("4".into()),
                    pc_banner_img: None,
                    h5_banner_img: None,
                    sort: None,
                    status: Some(4),
                    remark: Some("4".into()),
                    create_time: Some(DateTime::from_str("2023-10-10 00:00:00+08:00").unwrap()),
                    version: Some(4),
                    delete_flag: Some(4),
                    count: 4,
                },
            ];
            let r = MockTable::update_by_column_batch(&mut rb, &tables, "id", 2)
                .await
                .unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_select_all() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_all(&mut rb).await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{:?}", sql);
            assert_eq!(sql.trim(), "select * from mock_table");
        };
        block_on(f);
    }

    #[test]
    fn test_delete_by_column() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::delete_by_column(&mut rb, "1", &Value::String("1".to_string()))
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "delete from mock_table where 1 = ?");
            assert_eq!(args, vec![to_value!("1")]);
        };
        block_on(f);
    }

    #[test]
    fn test_delete_by_column_batch() {
        #[derive(Debug)]
        pub struct TestIntercept {
            pub num: AtomicI32,
        }

        #[async_trait]
        impl Intercept for TestIntercept {
            async fn before(
                &self,
                _task_id: i64,
                _rb: &dyn Executor,
                sql: &mut String,
                args: &mut Vec<Value>,
                _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
            ) -> Result<Option<bool>, Error> {
                if self.num.load(Ordering::Relaxed) == 0 {
                    assert_eq!(sql, "delete from mock_table where 1 in (?,?)");
                    assert_eq!(args, &vec![to_value!("1"), to_value!("2")]);
                } else {
                    assert_eq!(sql, "delete from mock_table where 1 in (?,?)");
                    assert_eq!(args, &vec![to_value!("3"), to_value!("4")]);
                }
                self.num.fetch_add(1, Ordering::Relaxed);
                return Ok(Some(true));
            }
        }
        let f = async move {
            let mut rb = RBatis::new();
            rb.set_intercepts(vec![Arc::new(TestIntercept {
                num: Default::default(),
            })]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::delete_by_column_batch(&mut rb, "1", &["1", "2", "3", "4"], 2)
                .await
                .unwrap();
        };
        block_on(f);
    }

    impl_select!(MockTable{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
    #[test]
    fn test_select_all_by_id() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_all_by_id(&mut rb, "1", "1")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table where id = ? and name = ?");
            assert_eq!(args, vec![to_value!("1"), to_value!("1")]);
        };
        block_on(f);
    }
    impl_select!(MockTable{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
    #[test]
    fn test_select_by_id() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_by_id(&mut rb, "1").await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table where id = ? limit 1");
            assert_eq!(args, vec![to_value!("1")]);
        };
        block_on(f);
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct DTO {
        id: String,
    }
    impl_select!(MockTable{select_by_dto(dto:DTO) -> Option => "`where id = '${dto.id}' limit 1`"});
    #[test]
    fn test_select_by_dto() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_by_dto(
                &mut rb,
                DTO {
                    id: "1".to_string(),
                },
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table where id = '1' limit 1");
        };
        block_on(f);
    }
    impl_update!(MockTable{update_by_name(name:&str) => "`where id = '2'`"});
    #[test]
    fn test_update_by_name() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let r = MockTable::update_by_name(&mut rb, &t, "test")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "update mock_table set id=?,name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=?,count=? where id = '2'");
            assert_eq!(
                args,
                vec![
                    to_value!(t.id),
                    to_value!(t.name),
                    to_value!(t.pc_link),
                    to_value!(t.h5_link),
                    to_value!(t.status),
                    to_value!(t.remark),
                    to_value!(t.create_time),
                    to_value!(t.version),
                    to_value!(t.delete_flag),
                    to_value!(t.count),
                ]
            );
        };
        block_on(f);
    }

    impl_update!(MockTable{update_by_dto(dto:DTO) => "`where id = '${dto.id}'`"});
    #[test]
    fn test_update_by_dto() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let r = MockTable::update_by_dto(
                &mut rb,
                &t,
                DTO {
                    id: "2".to_string(),
                },
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "update mock_table set id=?,name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,delete_flag=?,count=? where id = '2'");
            assert_eq!(
                args,
                vec![
                    to_value!(t.id),
                    to_value!(t.name),
                    to_value!(t.pc_link),
                    to_value!(t.h5_link),
                    to_value!(t.status),
                    to_value!(t.remark),
                    to_value!(t.create_time),
                    to_value!(t.version),
                    to_value!(t.delete_flag),
                    to_value!(t.count),
                ]
            );
        };
        block_on(f);
    }

    impl_delete!(MockTable {delete_by_name(name:&str) => "`where name= '2'`"});
    #[test]
    fn test_delete_by_name() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::delete_by_name(&mut rb, "2").await.unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "delete from mock_table where name= '2'");
        };
        block_on(f);
    }
    impl_select_page!(MockTable{select_page(name:&str) => "`order by create_time desc`"});
    #[test]
    fn test_select_page() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![
                Arc::new(PageIntercept::new()),
                Arc::new(MockIntercept::new(queue.clone())),
            ]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_page(&mut rb, &PageRequest::new(1, 10), "1")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(
                sql,
                "select * from mock_table order by create_time desc limit 0,10 "
            );
            let (sql, args) = queue.pop().unwrap();
            assert_eq!(
                sql,
                "select count(1) as count from mock_table order by create_time desc"
            );
        };
        block_on(f);
    }
    impl_select_page!(MockTable{select_page_by_name(name:&str,account:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});
    #[test]
    fn test_select_page_by_name() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![
                Arc::new(PageIntercept::new()),
                Arc::new(MockIntercept::new(queue.clone())),
            ]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_page_by_name(&mut rb, &PageRequest::new(1, 10), "", "")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table where name != '' limit 0,10 ");
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(
                sql,
                "select count(1) as count from mock_table where name != ''"
            );
        };
        block_on(f);
    }

    #[test]
    fn test_select_by_column() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_by_column(&mut rb, "id", "1")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql.trim(), "select * from mock_table  where id = ?");
            assert_eq!(args, vec![to_value!("1")]);
        };
        block_on(f);
    }

    impl_select!(MockTable{select_from_table_name_by_id(id:&str,table_name:&str) => "`where id = #{id}`"});

    #[test]
    fn test_select_from_table_name_by_id() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_from_table_name_by_id(&mut rb, "1", "mock_table2")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table2 where id = ?");
            assert_eq!(args, vec![to_value!("1")]);
        };
        block_on(f);
    }

    impl_select!(MockTable{select_table_column_from_table_name_by_id(id:&str,table_column:&str) => "`where id = #{id}`"});

    #[test]
    fn test_select_table_column_from_table_name_by_id() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_table_column_from_table_name_by_id(&mut rb, "1", "id,name")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select id,name from mock_table where id = ?");
            assert_eq!(args, vec![to_value!("1")]);
        };
        block_on(f);
    }

    #[test]
    fn test_select_in_column() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::select_in_column(&mut rb, "1", &["1", "2"])
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from mock_table  where 1 in (?,?)");
            assert_eq!(args, vec![to_value!("1"), to_value!("2")]);
        };
        block_on(f);
    }

    #[test]
    fn test_delete_in_column() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![Arc::new(MockIntercept::new(queue.clone()))]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = MockTable::delete_in_column(&mut rb, "1", &["1", "2"])
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "delete from mock_table where 1 in (?,?)");
            assert_eq!(args, vec![to_value!("1"), to_value!("2")]);
        };
        block_on(f);
    }

    #[test]
    fn test_tx() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(DateTime::now()),
                version: Some(1),
                delete_flag: Some(1),
                count: 0,
            };
            let mut tx = rb.acquire_begin().await.unwrap();
            let _r = MockTable::insert(&mut tx, &t).await.unwrap();

            let mut tx = rb
                .acquire_begin()
                .await
                .unwrap()
                .defer_async(|_tx| async {});
            let _r = MockTable::insert(&mut tx, &t).await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_pool_get() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            for _ in 0..100000 {
                let mut tx = rb.acquire().await.unwrap();
            }
            println!("done");
        };
        block_on(f);
    }

    #[test]
    fn test_pool_try_get() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let mut v = vec![];
            for _ in 0..100000 {
                match rb.try_acquire().await {
                    Ok(tx) => {
                        v.push(tx);
                    }
                    Err(e) => {}
                }
            }
            println!("done={}", v.len());
        };
        block_on(f);
    }

    rbatis::htmlsql_select_page!(htmlsql_select_page_by_name(name: &str) -> MockTable => r#"<select id="select_page_data">`select `<if test="do_count == true">`count(1) from table`</if><if test="do_count == false">`* from table limit ${page_no},${page_size}`</if></select>"#);
    #[test]
    fn test_htmlsql_select_page_by_name() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![
                Arc::new(PageIntercept::new()),
                Arc::new(MockIntercept::new(queue.clone())),
            ]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = htmlsql_select_page_by_name(&mut rb, &PageRequest::new(1, 10), "")
                .await
                .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select * from table limit 0,10");
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(sql, "select count(1) as count from table");
        };
        block_on(f);
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    pub struct PySqlSelectPageArg {
        pub name: String,
    }

    rbatis::pysql_select_page!(pysql_select_page(item: PySqlSelectPageArg,var1:&str) -> MockTable =>
    r#"`select `
      if do_count == true:
        ` count(1) as count `
      if do_count == false:
         ` * `
      `from activity where delete_flag = 0 and var1 = #{var1}`
        if item.name != '':
           ` and name=#{item.name}`"#);

    #[test]
    fn test_pysql_select_page() {
        let f = async move {
            let mut rb = RBatis::new();
            let queue = Arc::new(SyncVec::new());
            rb.set_intercepts(vec![
                Arc::new(PageIntercept::new()),
                Arc::new(MockIntercept::new(queue.clone())),
            ]);
            rb.init(MockDriver {}, "test").unwrap();
            let r = pysql_select_page(
                &mut rb,
                &PageRequest::new(1, 10),
                PySqlSelectPageArg {
                    name: "aaa".to_string(),
                },
                "test",
            )
            .await
            .unwrap();
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(
                sql,
                "select  * from activity where delete_flag = 0 and var1 = ? and name=? limit 0,10 "
            );
            let (sql, args) = queue.pop().unwrap();
            println!("{}", sql);
            assert_eq!(
                sql,
                "select count(1) as count from activity where delete_flag = 0 and var1 = ? and name=?"
            );
        };
        block_on(f);
    }
}
