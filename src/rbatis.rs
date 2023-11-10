use crate::executor::{RBatisConnExecutor, RBatisTxExecutor};
use crate::intercept_log::LogInterceptor;
use crate::plugin::intercept::Intercept;
use crate::snowflake::new_snowflake_id;
use crate::Error;
use dark_std::sync::SyncVec;
use rbdc::rt::tokio::sync::Mutex;
use log::LevelFilter;
use rbdc::pool::{Pool};
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use rbdc::pool::conn_manager::ConnManager;
use rbdc_pool_mobc::MobcPool;

/// RBatis engine
#[derive(Clone, Debug)]
pub struct RBatis {
    // the connection pool
    pub pool: Arc<OnceLock<Box<dyn Pool>>>,
    // intercept vec(default the intercepts[0] is a log interceptor)
    pub intercepts: Arc<SyncVec<Arc<dyn Intercept>>>,
}

impl Default for RBatis {
    fn default() -> RBatis {
        RBatis::new()
    }
}

///RBatis Options
pub struct RBatisOption {
    /// sql intercept vec chain(will move to RBatis)
    pub intercepts: SyncVec<Arc<dyn Intercept>>,
}

impl Default for RBatisOption {
    fn default() -> Self {
        Self {
            intercepts: {
                let intercepts = SyncVec::new();
                intercepts
                    .push(Arc::new(LogInterceptor::new(LevelFilter::Info)) as Arc<dyn Intercept>);
                intercepts
            },
        }
    }
}

impl RBatis {
    ///create an RBatis
    pub fn new() -> Self {
        return Self::new_with_opt(RBatisOption::default());
    }

    ///new RBatis from RBatisOption
    pub fn new_with_opt(option: RBatisOption) -> Self {
        return Self {
            pool: Arc::new(OnceLock::new()),
            intercepts: Arc::new({
                let result = SyncVec::new();
                for x in option.intercepts {
                    result.push(x);
                }
                result
            }),
        };
    }

    /// self.init(driver, url)? and self.try_acquire().await? a connection.
    /// default use MobcPool
    pub async fn link<Driver: rbdc::db::Driver + 'static>(
        &self,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        self.init(driver, url)?;
        self.try_acquire().await?;
        Ok(())
    }

    /// init pool.
    /// default is MobcPool,if you want other pool please use init_option
    pub fn init<Driver: rbdc::db::Driver + 'static>(
        &self,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        if url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let mut option = driver.default_option();
        option.set_uri(url)?;
        let pool = MobcPool::new(ConnManager::new_opt_box(Box::new(driver), option))?;
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }


    /// init pool by DBPoolOptions and Pool
    /// for example:
    ///```
    /// use rbatis::RBatis;
    /// use rbdc_pool_mobc::MobcPool;
    /// use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
    /// let rb=RBatis::new();
    ///
    /// let opts=SqliteConnectOptions::new();
    /// let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, MobcPool>(SqliteDriver{},opts);
    /// ```
    ///
    pub fn init_option<
        Driver: rbdc::db::Driver + 'static,
        ConnectOptions: rbdc::db::ConnectOptions,
        Pool: rbdc::pool::Pool + 'static,
    >(
        &self,
        driver: Driver,
        option: ConnectOptions,
    ) -> Result<(), Error> {
        let pool = Pool::new(ConnManager::new_opt_box(Box::new(driver), Box::new(option)))?;
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }

    pub fn init_pool<Pool: rbdc::pool::Pool + 'static>(&self, pool: Pool) -> Result<(), Error> {
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }

    #[deprecated(note = "please use init_option()")]
    pub fn init_opt<
        Driver: rbdc::db::Driver + 'static,
        ConnectOptions: rbdc::db::ConnectOptions,
    >(
        &self,
        driver: Driver,
        options: ConnectOptions,
    ) -> Result<(), Error> {
        self.init_option::<Driver, ConnectOptions, MobcPool>(driver, options)
    }

    /// set_intercepts for many
    pub fn set_intercepts(&mut self, arg: Vec<Arc<dyn Intercept>>) {
        self.intercepts = Arc::new(SyncVec::from(arg));
    }

    /// get conn pool
    ///
    /// can set option for example:
    /// ```rust
    /// use rbatis::RBatis;
    /// let rb = RBatis::new();
    /// //rb.init(rbdc_sqlite::driver::SqliteDriver {},"sqlite://target/sqlite.db");
    /// //rb.get_pool().unwrap().resize(10);
    /// ```
    pub fn get_pool(&self) -> Result<&dyn Pool, Error> {
        let p = self
            .pool
            .get()
            .ok_or_else(|| Error::from("[rbatis] rbatis pool not inited!"))?;
        return Ok(p.deref());
    }

    /// get driver type
    pub fn driver_type(&self) -> Result<&str, Error> {
        let pool = self.get_pool()?;
        Ok(pool.driver_type())
    }

    /// get an DataBase Connection used for the next step
    pub async fn acquire(&self) -> Result<RBatisConnExecutor, Error> {
        let pool = self.get_pool()?;
        let conn = pool.get().await?;
        return Ok(RBatisConnExecutor {
            id: new_snowflake_id(),
            conn: Mutex::new(Box::new(conn)),
            rb: self.clone(),
        });
    }

    /// try get an DataBase Connection used for the next step
    pub async fn try_acquire(&self) -> Result<RBatisConnExecutor, Error> {
        self.try_acquire_timeout(Duration::from_secs(0)).await
    }

    /// try get an DataBase Connection used for the next step
    pub async fn try_acquire_timeout(&self, d: Duration) -> Result<RBatisConnExecutor, Error> {
        let pool = self.get_pool()?;
        let conn = pool.get_timeout(d).await?;
        return Ok(RBatisConnExecutor {
            id: new_snowflake_id(),
            conn: Mutex::new(Box::new(conn)),
            rb: self.clone(),
        });
    }

    /// get an DataBase Connection,and call begin method,used for the next step
    pub async fn acquire_begin(&self) -> Result<RBatisTxExecutor, Error> {
        let pool = self.get_pool()?;
        let mut conn = pool.get().await?;
        conn.exec("begin", vec![]).await?;
        return Ok(RBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: Mutex::new(Box::new(conn)),
            rb: self.clone(),
            done: false,
        });
    }

    /// try get an DataBase Connection,and call begin method,used for the next step
    pub async fn try_acquire_begin(&self) -> Result<RBatisTxExecutor, Error> {
        let conn = self.try_acquire().await?;
        conn.exec("begin", vec![]).await?;
        return Ok(RBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: conn.conn,
            rb: self.clone(),
            done: false,
        });
    }

    /// is debug mode
    pub fn is_debug_mode(&self) -> bool {
        crate::decode::is_debug_mode()
    }

    /// get intercept from name
    /// the default name just like `let name = std::any::type_name::<LogInterceptor>()`
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let name = std::any::type_name::<MockIntercept>();
    ///  let intercept = rb.get_intercept_dyn(name);
    /// ```
    pub fn get_intercept_dyn(&self, name: &str) -> Option<&dyn Intercept> {
        for x in self.intercepts.iter() {
            if name == x.name() {
                return Some(x.as_ref());
            }
        }
        return None;
    }

    /// get intercept from name
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let intercept = rb.get_intercept::<MockIntercept>();
    /// ```
    pub fn get_intercept<T: Intercept>(&self) -> Option<&T> {
        let name = std::any::type_name::<T>();
        for item in self.intercepts.iter() {
            if name == item.name() {
                //this is safe,limit by T::name() == item.name
                let rf = item.as_ref();
                let call: &T = unsafe { std::mem::transmute_copy(&rf) };
                return Some(call);
            }
        }
        return None;
    }
}
