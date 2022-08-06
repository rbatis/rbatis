use crate::{MssqlConnectOptions, MssqlConnection};
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver, Placeholder};
use rbdc::Error;
use tiberius::Config;

#[derive(Debug)]
pub struct MssqlDriver {}

impl Driver for MssqlDriver {
    fn name(&self) -> &str {
        "mssql"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let cfg = Config::from_jdbc_string(&url).map_err(|e| Error::from(e.to_string()))?;
            let conn = MssqlConnection::establish(&cfg).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt = opt.downcast_ref::<MssqlConnectOptions>().unwrap();
        Box::pin(async move {
            let conn = MssqlConnection::establish(&opt.0).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(MssqlConnectOptions(Config::new()))
    }
}

impl Placeholder for MssqlDriver {
    fn exchange(&self, sql: &str) -> String {
        let mut last = ' ' as u8;
        let mut sql_bytes = sql.as_bytes().to_vec();
        let mut placeholder_idx = 1;
        let mut index = 0;
        loop {
            if index == sql_bytes.len() {
                break;
            }
            let x = sql_bytes[index];
            if x == '?' as u8 && last != '\\' as u8 {
                sql_bytes[index] = '@' as u8;
                sql_bytes.insert(index + 1, 'P' as u8); //P
                sql_bytes.insert(index + 2, '0' as u8 + placeholder_idx);
                last = '0' as u8 + placeholder_idx;
                placeholder_idx += 1;
            } else {
                last = x;
            }
            index += 1;
        }
        String::from_utf8(sql_bytes).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::driver::MssqlDriver;
    use rbdc::block_on;
    use rbdc::db::{Driver, Placeholder};
    use rbdc::pool::Pool;
    use rbs::{to_value, Value};
    use std::collections::BTreeMap;

    #[test]
    fn test_mssql_pool() {
        let task = async move {
            //jdbc:sqlserver://[serverName[\instanceName][:portNumber]][;property=value[;property=value]]
            let uri =
                "jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test";
            // let pool = Pool::new_url(MssqlDriver {}, "jdbc:sqlserver://SA:TestPass!123456@localhost:1433;database=test").unwrap();
            let pool = Pool::new_url(MssqlDriver {}, uri).unwrap();
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
    fn test_exchange() {
        let d = MssqlDriver {};
        let s = d.exchange("select * from table where id = ? age = ?");
        println!("{}", s);
        assert_eq!(s, "select * from table where id = @P1 age = @P2")
    }
}
