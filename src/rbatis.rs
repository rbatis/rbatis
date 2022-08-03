use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::borrow::BorrowMut;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use rbdc::db::ExecResult;
use crate::core::Error;
use crate::crud::CRUDTable;
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor, RbatisExecutor};
use crate::plugin::intercept::SqlIntercept;
use crate::plugin::log::{LogPlugin, RbatisLogPlugin};
use crate::snowflake::new_snowflake_id;
use crate::utils::error_util::ToResult;
use crate::utils::string_util;
use crate::wrapper::Wrapper;
use std::fmt::{Debug, Formatter};
use rbdc::pool::Pool;
use crate::sql::page::PagePlugin;

/// rbatis engine
// #[derive(Debug)]
pub struct Rbatis {
    // the connection pool,use OnceCell init this
    pub pool: OnceCell<Pool>,
    // sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    // log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
    // page_plugin
    pub page_plugin: Arc<Box<dyn PagePlugin>>
}

impl Debug for Rbatis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rbatis")
            .field("pool", &self.pool)
            .field("page_plugin", &self.page_plugin)
            .field("sql_intercepts", &self.sql_intercepts)
            .field("logic_plugin", &self.logic_plugin)
            .finish()
    }
}

impl Default for Rbatis {
    fn default() -> Rbatis {
        Rbatis::new()
    }
}

///Rbatis Options
#[derive(Debug)]
pub struct RbatisOption {
    /// sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    /// log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
}

impl Default for RbatisOption {
    fn default() -> Self {
        Self {
            sql_intercepts: vec![],
            log_plugin: Arc::new(Box::new(RbatisLogPlugin::default()) as Box<dyn LogPlugin>),
        }
    }
}

impl Rbatis {
    ///create an Rbatis
    pub fn new() -> Self {
        return Self::new_with_opt(RbatisOption::default());
    }

    ///new Rbatis from Option
    pub fn new_with_opt(option: RbatisOption) -> Self {
        return Self {
            pool: OnceCell::new(),
            sql_intercepts: option.sql_intercepts,
            log_plugin: option.log_plugin,
            page_plugin: option.page_plugin
        };
    }

    /// try return an new wrapper,if not call the link() method,it will be panic!
    pub fn new_wrapper(&self) -> Wrapper {
        let driver = self.driver_type();
        Wrapper::new(&driver.unwrap_or_else(|_| {
            panic!("[rbatis] .new_wrapper() method must be call .link(url) to init first!");
        }))
    }

    /// try return an new wrapper and set table formats,if not call the link() method,it will be panic!
    pub fn new_wrapper_table<T>(&self) -> Wrapper
    where
        T: CRUDTable,
    {
        let mut w = self.new_wrapper();
        let formats = T::formats(&self.driver_type().unwrap());
        w = w.set_formats(formats);
        return w;
    }

    /// link pool
    pub async fn link<Driver: rbdc::db::Driver>(&self, driver:Driver,url: &str) -> Result<(), Error> {
        if url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let mut option = driver.default_option();
        option.set_uri(url)?;
        return Ok(self.link_opt(driver, option).await?);
    }

    /// link pool by DBPoolOptions
    /// for example:
    ///
    pub async fn link_opt<Driver: rbdc::db::Driver + 'static, ConnectOptions: rbdc::db::ConnectOptions>(
        &self,
        driver: Driver,
        options: ConnectOptions,
    ) -> Result<(), Error> {
        let pool = Pool::new(driver, options).await?;
        self.pool.set(pool);
        return Ok(());
    }

    pub fn set_log_plugin(&mut self, arg: impl LogPlugin + 'static) {
        self.log_plugin = Arc::new(Box::new(arg));
    }

    pub fn set_page_plugin(&mut self, arg: impl PagePlugin + 'static) {
        self.page_plugin = Box::new(arg);
    }

    pub fn add_sql_intercept(&mut self, arg: impl SqlIntercept + 'static) {
        self.sql_intercepts.push(Box::new(arg));
    }

    pub fn set_sql_intercepts(&mut self, arg: Vec<Box<dyn SqlIntercept>>) {
        self.sql_intercepts = arg;
    }

    /// get conn pool
    pub fn get_pool(&self) -> Result<&Pool, Error> {
        let p = self.pool.get();
        if p.is_none() {
            return Err(Error::from("[rbatis] rbatis pool not inited!"));
        }
        return Ok(p.unwrap());
    }

    /// get driver type
    pub fn driver_type(&self) -> Result<&str, Error> {
        let pool = self.get_pool()?;
        Ok(pool.name())
    }

    /// get an DataBase Connection used for the next step
    pub async fn acquire(&self) -> Result<RBatisConnExecutor<'_>, Error> {
        let pool = self.get_pool()?;
        let conn = pool.acquire().await?;
        return Ok(RBatisConnExecutor {
            conn: conn,
            rb: &self,
        });
    }

    /// get an DataBase Connection,and call begin method,used for the next step
    pub async fn acquire_begin(&self) -> Result<RBatisTxExecutor<'_>, Error> {
        let pool = self.get_pool()?;
        let conn = pool.begin().await?;
        return Ok(RBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: conn,
            rb: &self,
        });
    }

    /// is debug mode
    pub fn is_debug_mode(&self) -> bool {
        if cfg!(feature = "debug_mode") {
            return true;
        }
        return false;
    }

    /// change ref to executor
    pub fn as_executor(&self) -> RbatisExecutor {
        self.into()
    }
}
