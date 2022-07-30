use mobc::{async_trait, Manager};
use crate::{block_on, Error};
use crate::db::{ConnectOptions, Driver};

/// RBDC pool
pub type Pool = mobc::Pool<RBDCManager>;

pub struct RBDCManager {
    driver: Box<dyn Driver>,
    opt: Box<dyn ConnectOptions>,
}

#[async_trait]
impl Manager for RBDCManager {
    type Connection = Box<dyn crate::db::Connection>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.driver.connect_opt(self.opt.as_ref()).await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.ping().await?;
        Ok(conn)
    }
}

impl RBDCManager {
    pub fn new<D: Driver + 'static>(d: D, url: &str) -> Result<Self, Error> {
        let mut opt = d.option_default();
        opt.set_uri(url)?;
        Ok(Self {
            driver: Box::new(d),
            opt,
        })
    }
    pub fn new_opt<D: Driver + 'static, Option: ConnectOptions>(d: D, o: Option) -> Result<Self, Error> {
        Ok(Self {
            driver: Box::new(d),
            opt: Box::new(o),
        })
    }
}

#[test]
fn test_pool() {}