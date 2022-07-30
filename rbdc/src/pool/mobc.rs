use std::ops::{Deref, DerefMut};
use mobc::{async_trait, Builder, Manager};
use crate::{block_on, Error};
use crate::db::{ConnectOptions, Driver};

/// RBDC pool
pub struct Pool {
    pub inner: mobc::Pool<RBDCManager>,
}

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

impl Deref for Pool {
    type Target = mobc::Pool<RBDCManager>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl Pool {
    pub fn new<D: Driver + 'static>(d: D, url: &str) -> Result<Self, Error> {
        let pool = Pool {
            inner: mobc::Pool::new(RBDCManager::new(d, url)?)
        };
        Ok(pool)
    }
    pub fn new_conn_opt<D: Driver + 'static, Option: ConnectOptions>(d: D, o: Option) -> Result<Self, Error> {
        let pool = Pool {
            inner: mobc::Pool::new(RBDCManager::new_opt(d, url)?)
        };
        Ok(pool)
    }
    pub fn builder() -> Builder<RBDCManager> {
        mobc::Pool::builder()
    }
}

#[test]
fn test_pool() {

}