use crate::executor::{RBatisConnExecutor, RBatisTxExecutor};
use crate::intercept_log::LogInterceptor;
use crate::plugin::intercept::Intercept;
use crate::snowflake::new_snowflake_id;
use crate::Error;
use dark_std::sync::SyncVec;
use rbdc::rt::tokio::sync::Mutex;
use log::LevelFilter;
use rbdc::db::{Connection, Driver};
use rbdc::pool::{ManagerPorxy, Pool};
use std::fmt::Debug;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

/// RBatis engine
#[derive(Clone, Debug)]
pub struct RBatis {
    // the connection pool
    pub pool: Arc<OnceLock<Pool>>,
    // intercept vec(default the intercepts[0] is a log interceptor)
    pub intercepts: Arc<SyncVec<Arc<dyn Intercept>>>,
}

#[deprecated(note = "please use RBatis replace this")]
pub type Rbatis = RBatis;

#[deprecated(note = "please use RBatisOption replace this")]
pub type RbatisOption = RBatisOption;

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
            conn: Mutex::new(Box::new(conn)),
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
    pub fn get_intercept(&self, name: &str) -> Option<&dyn Intercept> {
        for x in self.intercepts.iter() {
            if name == x.name() {
                return Some(x.as_ref());
            }
        }
        return None;
    }
}
