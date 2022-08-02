use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use mobc::{async_trait, Builder, Manager};
use crate::{block_on, Error};
use crate::db::{ConnectOptions, Driver};

/// RBDC pool
pub struct Pool {
    pub manager: Arc<RBDCManager>,
    pub inner: mobc::Pool<ManagerPorxy>,
}

#[derive(Clone)]
pub struct ManagerPorxy{
    pub inner:Arc<RBDCManager>
}

impl From<Arc<RBDCManager>> for ManagerPorxy{
    fn from(arg: Arc<RBDCManager>) -> Self {
        ManagerPorxy{
            inner:arg
        }
    }
}

impl Deref for ManagerPorxy{
    type Target = RBDCManager;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Pool {
    pub fn name(&self) -> &str {
        self.manager.name()
    }
}

#[derive(Debug)]
pub struct RBDCManager {
    pub driver: Box<dyn Driver>,
    pub option: Box<dyn ConnectOptions>,
}

#[async_trait]
impl Manager for ManagerPorxy {
    type Connection = Box<dyn crate::db::Connection>;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.driver.connect_opt(self.option.as_ref()).await
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        conn.ping().await?;
        Ok(conn)
    }
}

impl RBDCManager {
    pub fn new<D: Driver + 'static>(driver: D, url: &str) -> Result<Self, Error> {
        let mut option = driver.default_option();
        option.set_uri(url)?;
        Ok(Self {
            driver: Box::new(driver),
            option:option,
        })
    }
    pub fn new_opt<D: Driver + 'static, Option: ConnectOptions>(driver: D, option: Option) -> Self {
        Self {
            driver: Box::new(driver),
            option: Box::new(option),
        }
    }

    pub fn name(&self) -> &str {
        self.driver.name()
    }
}

impl Deref for Pool {
    type Target = mobc::Pool<ManagerPorxy>;

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
    pub fn new_url<Driver: crate::db::Driver + 'static>(d: Driver, url: &str) -> Result<Self, Error> {
        let manager=Arc::new(RBDCManager::new(d, url)?);
        let p= mobc::Pool::new(ManagerPorxy::from(manager.clone()));
        let pool = Pool {
            manager: manager,
            inner: p,
        };
        Ok(pool)
    }
    pub fn new<Driver: crate::db::Driver + 'static, ConnectOptions: crate::db::ConnectOptions>(d: Driver, o: ConnectOptions) -> Self {
        let manager= Arc::new(RBDCManager::new_opt(d, o));
        let p= mobc::Pool::new(ManagerPorxy::from(manager.clone()));
        let pool = Pool {
            manager: manager,
            inner: p,
        };
        pool
    }

    pub fn builder() -> Builder<ManagerPorxy> {
        mobc::Pool::builder()
    }
}

#[test]
fn test_pool() {}