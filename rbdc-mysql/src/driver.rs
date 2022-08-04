use std::any::Any;
use std::ops::Deref;
use crate::connection::MySqlConnection;
use crate::options::MySqlConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver, Placeholder};
use rbdc::Error;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
pub struct MysqlDriver {}

impl Driver for MysqlDriver {
    fn name(&self) -> &str {
        "mysql"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let conn = MySqlConnection::establish(&url.parse()?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt = opt.downcast_ref().unwrap();
        Box::pin(async move {
            let conn = MySqlConnection::establish(opt).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(MySqlConnectOptions::default())
    }
}

impl Placeholder for MysqlDriver{
    fn exchange(&self, sql: &str) -> String{
        sql.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::driver::MysqlDriver;
    use rbdc::db::Driver;
    use rbs::{to_value, Value};
    use std::collections::BTreeMap;
    use rbdc::block_on;
    use rbdc::pool::Pool;

    #[test]
    fn test_mysql_pool() {
        let task = async move {
            let pool = Pool::new_url(MysqlDriver {},"mysql://root:123456@localhost:3306/test").unwrap();
            std::thread::sleep(std::time::Duration::from_secs(2));
            let mut conn = pool.get().await.unwrap();
            let data = conn
                .get_values("select * from biz_activity", vec![])
                .await
                .unwrap();
            for mut x in data {
                println!("row: {}", x);
            }
        };
        block_on!(task);
    }

    #[test]
    fn test_mysql_rows() {
        let task = async move {
            let mut d = MysqlDriver {};
            let mut c = d
                .connect("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();
            let data = c
                .get_values("select * from biz_activity", vec![])
                .await
                .unwrap();
            for mut x in data {
                println!("row: {}", x);
            }
        };
        block_on!(task);
    }

    //
    // #[tokio::test]
    // async fn test_mysql_count() {
    //     let mut d = MysqlDriver {};
    //     let mut c = d
    //         .connect("mysql://root:123456@localhost:3306/test")
    //         .await
    //         .unwrap();
    //     let data = c
    //         .exec(
    //             "update biz_activity set pc_link = '111' where id  = '1'",
    //             vec![],
    //         )
    //         .await
    //         .unwrap();
    //     println!("{}", data);
    // }
    //

    #[test]
    fn test_mysql_param() {
        let task = async move {
            let mut d = MysqlDriver {};
            let mut c = d
                .connect("mysql://root:123456@localhost:3306/test")
                .await
                .unwrap();
            let param = vec![
                Value::String("http://www.test.com".to_string()),
                Value::U64(1658848837828).into_ext("Timestamp"),
                Value::String("12312".to_string()),
            ];
            println!("param => {}", Value::Array(param.clone()));
            let data = c
                .exec(
                    "update biz_activity set pc_link = ?,create_time = ? where id  = ?",
                    param,
                )
                .await
                .unwrap();
            println!("{}", data);
        };
        block_on!(task);
    }
}
