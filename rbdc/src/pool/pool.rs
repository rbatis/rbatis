use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use futures_core::future::BoxFuture;
use mobc::{async_trait, Builder, Manager};
use rbs::Value;
use crate::{block_on, Error};
use crate::db::{Connection, ConnectOptions, Driver, ExecResult, Row};

/// RBDC pool
pub struct Pool {
    pub manager: Arc<RBDCManager>,
    pub inner: mobc::Pool<ManagerPorxy>,
}

impl Debug for Pool{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pool")
            .field("manager",&self.manager)
            .finish()
    }
}

#[derive(Clone)]
pub struct ManagerPorxy {
    pub inner: Arc<RBDCManager>,
}

impl From<Arc<RBDCManager>> for ManagerPorxy {
    fn from(arg: Arc<RBDCManager>) -> Self {
        ManagerPorxy {
            inner: arg
        }
    }
}

impl Deref for ManagerPorxy {
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
            option: option,
        })
    }
    pub fn new_opt<D: Driver + 'static, Option: ConnectOptions>(driver: D, option: Option) -> Self {
        Self {
            driver: Box::new(driver),
            option: Box::new(option),
        }
    }

    pub fn new_opt_box(driver: Box<dyn Driver>, option: Box<dyn ConnectOptions>) -> Self {
        Self {
            driver: driver,
            option: option,
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
        let manager = Arc::new(RBDCManager::new(d, url)?);
        let p = mobc::Pool::new(ManagerPorxy::from(manager.clone()));
        let pool = Pool {
            manager: manager,
            inner: p,
        };
        Ok(pool)
    }
    pub fn new<Driver: crate::db::Driver + 'static, ConnectOptions: crate::db::ConnectOptions>(d: Driver, o: ConnectOptions) -> Self {
        let manager = Arc::new(RBDCManager::new_opt(d, o));
        let p = mobc::Pool::new(ManagerPorxy::from(manager.clone()));
        let pool = Pool {
            manager: manager,
            inner: p,
        };
        pool
    }

    pub fn new_box(d: Box<dyn Driver>, o: Box<dyn ConnectOptions>) -> Self {
        let manager = Arc::new(RBDCManager::new_opt_box(d, o));
        let p = mobc::Pool::new(ManagerPorxy::from(manager.clone()));
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

impl Connection for mobc::Connection<ManagerPorxy>{
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.deref_mut().get_rows(sql,params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.deref_mut().exec(sql,params)
    }

    fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
        self.deref_mut().close()
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().ping()
    }
}

#[test]
fn test_pool() {}