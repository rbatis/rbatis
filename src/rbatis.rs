use std::borrow::BorrowMut;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::OnceCell;
use rbatis_core::db::DBConnectOption;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use uuid::Uuid;

use crate::core::db::{DBExecResult, DBPool, DBPoolConn, DBPoolOptions, DBQuery, DBTx, DriverType};
use crate::core::Error;
use crate::crud::CRUDTable;
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor, RbatisExecutor};
use crate::plugin::intercept::SqlIntercept;
use crate::plugin::log::{LogPlugin, RbatisLogPlugin};
use crate::plugin::logic_delete::{LogicDelete, RbatisLogicDeletePlugin};
use crate::plugin::page::{IPage, IPageRequest, Page, PagePlugin, RbatisPagePlugin};
use crate::sql::PageLimit;
use crate::utils::error_util::ToResult;
use crate::utils::string_util;
use crate::wrapper::Wrapper;
use std::fmt::{Debug, Formatter};
use crate::snowflake::new_snowflake_id;

/// rbatis engine
// #[derive(Debug)]
pub struct Rbatis {
    // the connection pool,use OnceCell init this
    pub pool: OnceCell<DBPool>,
    // page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    // sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    // log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
    // logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
    // sql param binder
    pub encoder: fn(q: &mut DBQuery, arg: bson2::Bson) -> crate::Result<()>,
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
    /// page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    /// sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    /// log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
    /// logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
}

impl Default for RbatisOption {
    fn default() -> Self {
        Self {
            page_plugin: Box::new(RbatisPagePlugin::new()),
            sql_intercepts: vec![],
            logic_plugin: None,
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
            page_plugin: option.page_plugin,
            sql_intercepts: option.sql_intercepts,
            logic_plugin: option.logic_plugin,
            log_plugin: option.log_plugin,
            encoder: |q, arg| {
                q.bind_value(arg)?;
                Ok(())
            },
        };
    }

    /// try return an new wrapper,if not call the link() method,it will be panic!
    pub fn new_wrapper(&self) -> Wrapper {
        let driver = self.driver_type();
        if driver.as_ref().unwrap().eq(&DriverType::None) {
            panic!("[rbatis] .new_wrapper() method must be call .link(url) to init first!");
        }
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
    pub async fn link(&self, driver_url: &str) -> Result<(), Error> {
        return Ok(self.link_opt(driver_url, DBPoolOptions::default()).await?);
    }

    /// link pool by DBPoolOptions
    /// for example:
    ///          let mut opt = PoolOptions::new();
    ///          opt.max_size = 20;
    ///          rb.link_opt("mysql://root:123456@localhost:3306/test", opt).await.unwrap();
    pub async fn link_opt(
        &self,
        driver_url: &str,
        pool_options: DBPoolOptions,
    ) -> Result<(), Error> {
        if driver_url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        let pool = DBPool::new_opt_str(driver_url, pool_options).await?;
        self.pool.set(pool);
        return Ok(());
    }

    /// link pool by DBConnectOption and DBPoolOptions
    /// for example:
    ///         let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
    ///         rb.link_cfg(&db_cfg,PoolOptions::new());
    pub async fn link_cfg(
        &self,
        connect_option: &DBConnectOption,
        pool_options: DBPoolOptions,
    ) -> Result<(), Error> {
        let pool = DBPool::new_opt(connect_option, pool_options).await?;
        self.pool.set(pool);
        return Ok(());
    }

    pub fn set_log_plugin(&mut self, arg: impl LogPlugin + 'static) {
        self.log_plugin = Arc::new(Box::new(arg));
    }

    pub fn set_logic_plugin(&mut self, arg: impl LogicDelete + 'static) {
        self.logic_plugin = Some(Box::new(arg));
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
    pub fn get_pool(&self) -> Result<&DBPool, Error> {
        let p = self.pool.get();
        if p.is_none() {
            return Err(Error::from("[rbatis] rbatis pool not inited!"));
        }
        return Ok(p.unwrap());
    }

    /// get driver type
    pub fn driver_type(&self) -> Result<DriverType, Error> {
        let pool = self.get_pool()?;
        Ok(pool.driver_type())
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


pub trait AsSqlTag {
    fn sql_tag(&self) -> char;
    fn do_replace_tag(&self, sql: &mut String);
}

impl AsSqlTag for DriverType {
    #[inline]
    fn sql_tag(&self) -> char {
        match self {
            DriverType::None => { '?' }
            DriverType::Mysql => { '?' }
            DriverType::Sqlite => { '?' }
            DriverType::Postgres => { '$' }
            //mssql is '@p',so use '$' to '@p'
            DriverType::Mssql => { '$' }
        }
    }
    #[inline]
    fn do_replace_tag(&self, sql: &mut String) {
        if self.eq(&DriverType::Mssql) {
            *sql = sql.replace("$", "@p");
        }
    }
}


