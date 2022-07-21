use crate::connection::PgConnection;
use crate::options::PgConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection, Driver};
use rbdc::Error;
use std::str::FromStr;

pub struct PgDriver {}

impl Driver for PgDriver {
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let conn = PgConnection::establish(&url.parse()?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn make_option(&self, url: &str) -> Result<Box<dyn ConnectOptions>, Error> {
        Ok(Box::new(PgConnectOptions::from_str(&url)?))
    }
}

#[cfg(test)]
mod test {
    use crate::driver::PgDriver;
    use rbdc::db::Driver;
    use rbdc::pool::PoolOptions;

    #[tokio::test]
    async fn test_pg_pool() {
        let opt = PoolOptions::new();
        let pool = opt
            .connect(
                Box::new(PgDriver {}),
                "postgres://postgres:123456@localhost:5432/postgres",
            )
            .await
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("{:?}", pool);
        let mut conn = pool.acquire().await.unwrap();
        let data = conn
            .get_values("select * from biz_activity", vec![])
            .await
            .unwrap();
        for mut x in data {
            println!("row: {}", x);
        }
    }
}
