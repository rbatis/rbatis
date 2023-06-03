use crate::executor::{RBatisConnExecutor, RBatisTxExecutor};
use crate::plugin::intercept::SqlIntercept;
use crate::plugin::log::{LogPlugin, RBatisLogPlugin};
use crate::snowflake::new_snowflake_id;
use crate::Error;
use dark_std::sync::SyncVec;
use rbdc::db::Connection;
use rbdc::pool::{ManagerPorxy, Pool};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use crate::intercept::LogInterceptor;

/// RBatis engine
#[derive(Clone)]
pub struct RBatis {
    // the connection pool,use OnceCell init this
    pub pool: Arc<OnceLock<Pool>>,
    // sql intercept vec chain
    pub sql_intercepts: Arc<SyncVec<Box<dyn SqlIntercept>>>,
    // log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
}

#[deprecated(note = "please use RBatis replace this")]
pub type Rbatis = RBatis;

impl Debug for RBatis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatis")
            .field("pool", &self.pool)
            .field("sql_intercepts", &self.sql_intercepts.len())
            .finish()
    }
}

impl Default for RBatis {
    fn default() -> RBatis {
        RBatis::new()
    }
}

///RBatis Options
pub struct RBatisOption {
    /// sql intercept vec chain
    pub sql_intercepts: SyncVec<Box<dyn SqlIntercept>>,
    /// log plugin
    pub log_plugin: Box<dyn LogPlugin>,
}

impl Default for RBatisOption {
    fn default() -> Self {
        Self {
            sql_intercepts: {
                let intercepts = SyncVec::new();
                intercepts.push(Box::new(LogInterceptor {}) as Box<dyn SqlIntercept>);
                intercepts
            },
            log_plugin: Box::new(RBatisLogPlugin::default()) as Box<dyn LogPlugin>,
        }
    }
}

impl RBatis {
    ///create an RBatis
    pub fn new() -> Self {
        return Self::new_with_opt(RBatisOption::default());
    }

    ///new RBatis from Option
    pub fn new_with_opt(option: RBatisOption) -> Self {
        return Self {
            pool: Arc::new(OnceLock::new()),
            sql_intercepts: Arc::new(option.sql_intercepts),
            log_plugin: Arc::new(option.log_plugin),
        };
    }

    /// self.init(driver, url)? and self.try_acquire().await? a connection.
    pub async fn link<Driver: rbdc::db::Driver + 'static>(
        &self,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        self.init(driver, url)?;
        self.try_acquire().await?;
        Ok(())
    }

    /// init pool
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
        let pool = Pool::new_box(Box::new(driver), option)?;
        self.pool
            .set(pool)
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }

    /// init pool
    pub async fn init_builder<Driver: rbdc::db::Driver + 'static>(
        &self,
        builder: rbdc::deadpool::managed::PoolBuilder<
            ManagerPorxy,
            rbdc::deadpool::managed::Object<ManagerPorxy>,
        >,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        if url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let mut option = driver.default_option();
        option.set_uri(url)?;
        let pool = Pool::new_builder(builder, Box::new(driver), option)?;
        self.pool
            .set(pool)
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }

    /// init pool by DBPoolOptions
    /// for example:
    ///
    pub fn init_opt<
        Driver: rbdc::db::Driver + 'static,
        ConnectOptions: rbdc::db::ConnectOptions,
    >(
        &self,
        driver: Driver,
        options: ConnectOptions,
    ) -> Result<(), Error> {
        let pool = Pool::new(driver, options)?;
        self.pool
            .set(pool)
            .map_err(|_e| Error::from("pool set fail!"))?;
        return Ok(());
    }

    /// set_log_plugin
    pub fn set_log_plugin(&mut self, arg: impl LogPlugin + 'static) {
        self.log_plugin = Arc::new(Box::new(arg));
    }

    /// set_sql_intercepts for many
    pub fn set_sql_intercepts(&mut self, arg: Vec<Box<dyn SqlIntercept>>) {
        self.sql_intercepts = Arc::new(SyncVec::from(arg));
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
    pub fn get_pool(&self) -> Result<&Pool, Error> {
        let p = self
            .pool
            .get()
            .ok_or_else(|| Error::from("[rbatis] rbatis pool not inited!"))?;
        return Ok(p);
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
            conn: Box::new(conn),
            rb: self.clone(),
        });
    }

    /// try get an DataBase Connection used for the next step
    pub async fn try_acquire(&self) -> Result<RBatisConnExecutor, Error> {
        let pool = self.get_pool()?;
        let mut default = pool.inner.timeouts().clone();
        default.wait = Some(Duration::ZERO);
        let conn = pool.timeout_get(&default).await?;
        return Ok(RBatisConnExecutor {
            conn: Box::new(conn),
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
            conn: Box::new(conn),
            rb: self.clone(),
            done: false,
        });
    }

    /// try get an DataBase Connection,and call begin method,used for the next step
    pub async fn try_acquire_begin(&self) -> Result<RBatisTxExecutor, Error> {
        let mut conn = self.try_acquire().await?;
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
}
