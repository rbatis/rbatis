use futures_core::future::BoxFuture;
use tiberius::Config;
use rbdc::db::{Connection, ConnectOptions, Driver};
use rbdc::Error;
use crate::{MssqlConnection, MssqlConnectOptions};

#[derive(Debug)]
pub struct MssqlDriver {}

impl Driver for MssqlDriver {
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let cfg=Config::from_jdbc_string(&url).map_err(|e|Error::from(e.to_string()))?;
            let conn = MssqlConnection::establish(
                &cfg
            ).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt = opt.downcast_ref::<MssqlConnectOptions>().unwrap();
        Box::pin(async move {
            let conn = MssqlConnection::establish(&opt.0).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn option_default(&self) -> Box<dyn ConnectOptions> {
        Box::new(MssqlConnectOptions(Config::new()))
    }
}

#[cfg(test)]
mod test {
    use rbdc::db::Driver;
    use rbs::{to_value, Value};
    use std::collections::BTreeMap;
    use rbdc::block_on;
    use rbdc::pool::Pool;
    use crate::driver::MssqlDriver;

    #[test]
    fn test_mssql_pool() {
        let task = async move {
            let pool = Pool::new_url(MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
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
}