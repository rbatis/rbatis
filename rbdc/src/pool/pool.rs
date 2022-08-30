use crate::db::{ConnectOptions, Connection, Driver, ExecResult, Row};
use crate::Error;
use async_trait::async_trait;
use deadpool::managed::{
    Manager, Object, PoolBuilder, PoolError, RecycleError, RecycleResult, Timeouts,
};
use deadpool::Status;
use futures_core::future::BoxFuture;
use rbs::Value;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;

/// RBDC pool.
/// you can use just like any deadpool methods  pool.deref().close() and more...
#[derive(Clone)]
pub struct Pool {
    pub manager: ManagerPorxy,
    pub inner: deadpool::managed::Pool<ManagerPorxy>,
}

impl Pool {
    /// return driver name
    pub fn driver_type(&self) -> &str {
        self.manager.driver_type()
    }

    /// spawn task on runtime
    pub fn spawn_task<T>(&self, task: T)
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        self.manager.spawn_task(task)
    }

    /**
     * Resize the pool. This change the `max_size` of the pool dropping
     * excess objects and/or making space for new ones.
     *
     * If the pool is closed this method does nothing. The [`Pool::status`] method
     * always reports a `max_size` of 0 for closed pools.
     */
    pub fn resize(&self, max_size: usize) {
        self.deref().resize(max_size);
    }

    /// Indicates whether this [`Pool`] has been closed.
    pub fn is_closed(&self) -> bool {
        self.deref().is_closed()
    }

    /// Closes this Pool.
    /// All current and future tasks waiting for Objects will return PoolError::Closed immediately.
    /// This operation resizes the pool to 0.
    pub fn close(&self) {
        self.deref().close();
    }

    /// Retrieves Status of this Pool.
    pub fn status(&self) -> Status {
        self.deref().status()
    }

    ///Get current timeout configuration
    pub fn timeouts(&self) -> Timeouts {
        self.deref().timeouts()
    }

    /// get connection
    pub async fn get(&self) -> Result<Object<ManagerPorxy>, PoolError<Error>> {
        self.deref().get().await
    }

    /// try get connection
    pub async fn try_get(&self) -> Result<Object<ManagerPorxy>, PoolError<Error>> {
        let mut t = self.deref().timeouts();
        t.wait = Some(Duration::ZERO);
        self.deref().timeout_get(&t).await
    }
}

impl Debug for Pool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pool")
            .field("manager", &self.manager)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct ManagerPorxy {
    pub inner: Arc<RBDCManager>,
}

impl ManagerPorxy {
    /// spawn task on runtime
    pub fn spawn_task<T>(&self, task: T)
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        tokio::spawn(task);
    }
}

impl From<Arc<RBDCManager>> for ManagerPorxy {
    fn from(arg: Arc<RBDCManager>) -> Self {
        ManagerPorxy { inner: arg }
    }
}

impl Deref for ManagerPorxy {
    type Target = RBDCManager;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct RBDCManager {
    pub driver: Box<dyn Driver>,
    pub option: Box<dyn ConnectOptions>,
}

pub struct DropBox {
    pub manager_proxy: ManagerPorxy,
    pub conn: Option<Box<dyn Connection>>,
}

impl Deref for DropBox {
    type Target = Box<dyn Connection>;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl DerefMut for DropBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for DropBox {
    fn drop(&mut self) {
        if let Some(mut conn) = self.conn.take() {
            self.manager_proxy.spawn_task(async move {
                let _ = conn.close().await;
            });
        }
    }
}

#[async_trait]
impl Manager for ManagerPorxy {
    type Type = DropBox;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(DropBox {
            manager_proxy: self.clone(),
            conn: Some(self.driver.connect_opt(self.option.as_ref()).await?),
        })
    }

    async fn recycle(&self, conn: &mut Self::Type) -> RecycleResult<Self::Error> {
        match conn.ping().await {
            Ok(_) => Ok(()),
            Err(e) => {
                //shutdown connection
                if let Some(mut conn) = conn.conn.take() {
                    let _ = conn.close().await;
                }
                return Err(RecycleError::Message(format!(
                    "Connection is ping fail={}",
                    e
                )));
            }
        }
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

    pub fn driver_type(&self) -> &str {
        self.driver.name()
    }
}

impl Deref for Pool {
    type Target = deadpool::managed::Pool<ManagerPorxy>;

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
    pub fn new_url<Driver: crate::db::Driver + 'static>(
        d: Driver,
        url: &str,
    ) -> Result<Self, Error> {
        let manager = ManagerPorxy::from(Arc::new(RBDCManager::new(d, url)?));
        let p = Pool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        let pool = Pool {
            manager: manager,
            inner: p,
        };
        Ok(pool)
    }
    pub fn new<Driver: crate::db::Driver + 'static, ConnectOptions: crate::db::ConnectOptions>(
        d: Driver,
        o: ConnectOptions,
    ) -> Result<Self, Error> {
        let manager = ManagerPorxy::from(Arc::new(RBDCManager::new_opt(d, o)));
        let inner = Pool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(Pool {
            manager: manager,
            inner: inner,
        })
    }

    pub fn new_box(d: Box<dyn Driver>, o: Box<dyn ConnectOptions>) -> Result<Self, Error> {
        let manager = ManagerPorxy::from(Arc::new(RBDCManager::new_opt_box(d, o)));
        let inner = Pool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(Pool {
            manager: manager,
            inner: inner,
        })
    }

    pub fn new_builder(
        builder: PoolBuilder<ManagerPorxy, Object<ManagerPorxy>>,
        d: Box<dyn Driver>,
        o: Box<dyn ConnectOptions>,
    ) -> Result<Self, Error> {
        let manager = ManagerPorxy::from(Arc::new(RBDCManager::new_opt_box(d, o)));
        Ok(Pool {
            manager: manager,
            inner: builder.build().map_err(|e| Error::from(e.to_string()))?,
        })
    }

    pub fn builder(m: ManagerPorxy) -> PoolBuilder<ManagerPorxy, Object<ManagerPorxy>> {
        deadpool::managed::Pool::builder(m)
    }
}

impl Connection for Object<ManagerPorxy> {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.deref_mut().get_rows(sql, params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.deref_mut().exec(sql, params)
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().close()
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().ping()
    }
}

#[test]
fn test_pool() {}
