use crate::connection::MySqlConnection;
use crate::options::MySqlConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver};
use rbdc::Error;
use std::pin::Pin;
use std::str::FromStr;

pub struct MysqlDriver {}

impl Driver for MysqlDriver {
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let conn = MySqlConnection::establish(&url.parse()?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn make_option(&self, url: &str) -> Result<Box<dyn ConnectOptions>, Error> {
        Ok(Box::new(MySqlConnectOptions::from_str(&url)?))
    }
}

#[cfg(test)]
mod test {
    use crate::driver::MysqlDriver;
    use rbdc::db::Driver;
    use rbdc::pool::PoolOptions;
    use rbs::Value;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_mysql_pool() {
        let opt = PoolOptions::new();
        let pool = opt
            .connect(
                Box::new(MysqlDriver {}),
                "mysql://root:123456@localhost:3306/test",
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_mysql_rows() {
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
    // #[tokio::test]
    // async fn test_mysql_param() {
    //     let mut d = MysqlDriver {};
    //     let mut c = d
    //         .connect("mysql://root:123456@localhost:3306/test")
    //         .await
    //         .unwrap();
    //     let param = vec![
    //         Value::String("http://www.test.com".to_string()),
    //         Value::Map(vec![(
    //             Value::String("datetime".to_string()),
    //             Value::String("2022-08-08T22:39:11.666667".to_string()),
    //         )]),
    //         Value::String("12312".to_string()),
    //     ];
    //     println!("param => {}", Value::Array(param.clone()));
    //     let data = c
    //         .exec(
    //             "update biz_activity set pc_link = ?,create_time = ? where id  = ?",
    //             param,
    //         )
    //         .await
    //         .unwrap();
    //     println!("{}", data);
    // }
}
