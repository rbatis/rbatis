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
            let conn = MySqlConnection::establish(&MySqlConnectOptions::from_str(&url)?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    // fn connect_opt(
    //     &self,
    //     opt: &dyn ConnectOptions,
    // ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
    //     Box::pin(async move {
    //         let conn = MySqlConnection::establish(opt).await?;
    //         Ok(Box::new(conn) as Box<dyn Connection>)
    //     })
    // }
}

#[cfg(test)]
mod test {
    use crate::driver::MysqlDriver;
    use rbdc::db::Driver;
    use rbs::Value;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_mysql_rows() {
        let mut d = MysqlDriver {};
        let mut c = d
            .connect("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let data = c
            .exec_rows("select * from biz_activity", vec![])
            .await
            .unwrap();
        for x in data {
            let md = x.meta_data();
            let mut m = vec![];
            for i in 0..md.column_len() {
                let n = md.column_name(i);
                // println!("column:{}", n);
                // println!("column value:{}", x.get(i).unwrap_or(Value::Nil));
                m.push((Value::String(n), x.get(i).unwrap_or(Value::Nil)));
            }
            println!("row: {}", Value::Map(m));
        }
    }

    #[tokio::test]
    async fn test_mysql_count() {
        let mut d = MysqlDriver {};
        let mut c = d
            .connect("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let data = c
            .exec(
                "update biz_activity set pc_link = '111' where id  = '1'",
                vec![],
            )
            .await
            .unwrap();
        println!("{}", data);
    }
}
