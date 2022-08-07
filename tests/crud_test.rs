#[macro_use]
extern crate rbatis;

#[cfg(test)]
mod test {
    use std::any::Any;
    use futures_core::future::BoxFuture;
    use rbatis::{Error, Rbatis};
    use rbdc::block_on;
    use rbdc::datetime::FastDateTime;
    use rbdc::db::{Connection, ConnectOptions, Driver, ExecResult, MetaData, Row};
    use rbdc::rt::block_on;
    use rbs::Value;

    #[derive(Debug, Clone)]
    pub struct MockDriver {}

    impl Driver for MockDriver {
        fn name(&self) -> &str {
            "test"
        }

        fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn default_option(&self) -> Box<dyn ConnectOptions> {
            Box::new(MockConnectOptions {})
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockRowMetaData {}

    impl MetaData for MockRowMetaData {
        fn column_len(&self) -> usize {
            1
        }

        fn column_name(&self, i: usize) -> String {
            "sql".to_string()
        }

        fn column_type(&self, i: usize) -> String {
            "String".to_string()
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockRow {
        pub sql: String,
    }

    impl Row for MockRow {
        fn meta_data(&self) -> Box<dyn MetaData> {
            Box::new(MockRowMetaData {}) as Box<dyn MetaData>
        }

        fn get(&mut self, i: usize) -> Option<Value> {
            Some(Value::String(self.sql.clone()))
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockConnection {}

    impl Connection for MockConnection {
        fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
            let sql = sql.to_string();
            Box::pin(async move {
                let data = Box::new(MockRow {
                    sql: sql
                }) as Box<dyn Row>;
                Ok(vec![data])
            })
        }

        fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
            let sql = sql.to_string();
            Box::pin(async move {
                Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::String(sql.to_string()),
                })
            })
        }

        fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
            Box::pin(async {
                Ok(())
            })
        }

        fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
            Box::pin(async {
                Ok(())
            })
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockConnectOptions {}

    impl ConnectOptions for MockConnectOptions {
        fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
            Box::pin(async {
                Ok(Box::new(MockConnection {}) as Box<dyn Connection>)
            })
        }

        fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
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
        pub create_time: Option<rbdc::datetime::FastDateTime>,
        pub version: Option<i64>,
        pub sql: String,
        pub delete_flag: Option<i32>,
    }
    crud!(MockTable {});
    impl_select!(MockTable{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
    impl_select!(MockTable{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
    impl_update!(MockTable{update_by_name(name:&str)} => "`where id = '2'`");
    impl_delete!(MockTable {delete_by_name(name:&str)} => "`where name= '2'`");
    impl_select_page!(MockTable{select_page() => "`order by create_time desc`"});
    impl_select_page!(MockTable{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

    #[test]
    fn test_insert() {
        let f = async move {
            let mut rb = Rbatis::new();
            rb.link(MockDriver {}, "test").await.unwrap();
            let mut t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(FastDateTime::now()),
                version: Some(1),
                sql: "".to_string(),
                delete_flag: Some(1),
            };
            let r = MockTable::insert(&mut rb, &t).await.unwrap();
            println!("{}", r.last_insert_id.as_str().unwrap_or_default());
            assert_eq!(r.last_insert_id.as_str().unwrap_or_default(),"insert into mock_table (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,sql,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)");
        };
        block_on(f);
    }
    #[test]
    fn test_update_by_column() {
        let f = async move {
            let mut rb = Rbatis::new();
            rb.link(MockDriver {}, "test").await.unwrap();
            let mut t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(FastDateTime::now()),
                version: Some(1),
                sql: "".to_string(),
                delete_flag: Some(1),
            };
            let r = MockTable::update_by_column(&mut rb, &t, "id").await.unwrap();
            println!("{}", r.last_insert_id.as_str().unwrap_or_default());
            assert_eq!(r.last_insert_id.as_str().unwrap_or_default(),"update mock_table set name=?,pc_link=?,h5_link=?,status=?,remark=?,create_time=?,version=?,sql=?,delete_flag=? where  id = ?");
        };
        block_on(f);
    }
    #[test]
    fn test_select_all() {
        let f = async move {
            let mut rb = Rbatis::new();
            rb.link(MockDriver {}, "test").await.unwrap();
            let mut t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(FastDateTime::now()),
                version: Some(1),
                sql: "".to_string(),
                delete_flag: Some(1),
            };
            let r = MockTable::select_all(&mut rb).await.unwrap();
            println!("{:?}", r[0].sql);
            assert_eq!(r[0].sql,"select * from mock_table");
        };
        block_on(f);
    }
    #[test]
    fn test_delete_by_column() {
        let f = async move {
            let mut rb = Rbatis::new();
            rb.link(MockDriver {}, "test").await.unwrap();
            let mut t = MockTable {
                id: Some("2".into()),
                name: Some("2".into()),
                pc_link: Some("2".into()),
                h5_link: Some("2".into()),
                pc_banner_img: None,
                h5_banner_img: None,
                sort: None,
                status: Some(2),
                remark: Some("2".into()),
                create_time: Some(FastDateTime::now()),
                version: Some(1),
                sql: "".to_string(),
                delete_flag: Some(1),
            };
            let r = MockTable::delete_by_column(&mut rb, "1", &Value::String("1".to_string())).await.unwrap();
            println!("{}", r.last_insert_id.as_str().unwrap_or_default());
            assert_eq!(r.last_insert_id.as_str().unwrap_or_default(),"delete from mock_table where  1 = ?");
        };
        block_on(f);
    }
}