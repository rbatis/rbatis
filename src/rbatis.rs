use std::borrow::BorrowMut;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use once_cell::sync::OnceCell;
use py_sql::node::proxy_node::NodeFactory;
use py_sql::py_sql::PyRuntime;
use rbatis_core::db::DBConnectOption;
use rexpr::runtime::RExprRuntime;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Number;
use uuid::Uuid;

use crate::core::db::{DBExecResult, DBPool, DBPoolConn, DBPoolOptions, DBQuery, DBTx, DriverType};
use crate::core::Error;
use crate::crud::CRUDTable;
use crate::executor::{RBatisConnExecutor, RBatisTxExecutor};
use crate::plugin::intercept::SqlIntercept;
use crate::plugin::log::{LogPlugin, RbatisLogPlugin};
use crate::plugin::logic_delete::{LogicDelete, RbatisLogicDeletePlugin};
use crate::plugin::page::{IPage, IPageRequest, Page, PagePlugin, RbatisPagePlugin};
use crate::plugin::version_lock::{RbatisVersionLockPlugin, VersionLockPlugin};
use crate::sql::PageLimit;
use crate::tx::{TxGuard, TxManager};
use crate::utils::error_util::ToResult;
use crate::utils::string_util;
use crate::wrapper::Wrapper;

/// rbatis engine
#[derive(Debug)]
pub struct Rbatis {
    // the connection pool,use OnceCell init this
    pub pool: OnceCell<DBPool>,
    // the runtime run some express for example:'1+1'=2
    pub runtime_expr: RExprRuntime,
    //py lang runtime run some express for py_sql
    pub runtime_py: PyRuntime,
    //tx manager
    pub tx_manager: Arc<TxManager>,
    // page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    // sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    // logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
    // log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
    // version lock plugin
    pub version_lock_plugin: Option<Box<dyn VersionLockPlugin>>,
}

impl Drop for Rbatis {
    fn drop(&mut self) {
        self.tx_manager.set_alive(false);
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
    /// the tx lock timeout, if out of time tx will be rollback
    pub tx_lock_wait_timeout: Duration,
    /// rbatis tx manager check tx interval
    pub tx_check_interval: Duration,
    /// custom py lang
    pub generate: Vec<Box<dyn NodeFactory>>,
    /// page plugin
    pub page_plugin: Box<dyn PagePlugin>,
    /// sql intercept vec chain
    pub sql_intercepts: Vec<Box<dyn SqlIntercept>>,
    /// logic delete plugin
    pub logic_plugin: Option<Box<dyn LogicDelete>>,
    /// log plugin
    pub log_plugin: Arc<Box<dyn LogPlugin>>,
    ///tx_prefix,default is 'tx:'
    pub tx_prefix: String,
    ///version lock plugin
    pub version_lock_plugin: Option<Box<dyn VersionLockPlugin>>,
}

impl Default for RbatisOption {
    fn default() -> Self {
        Self {
            tx_lock_wait_timeout: Duration::from_secs(60),
            tx_check_interval: Duration::from_secs(1),
            generate: vec![],
            page_plugin: Box::new(RbatisPagePlugin::new()),
            sql_intercepts: vec![],
            logic_plugin: None,
            log_plugin: Arc::new(Box::new(RbatisLogPlugin::default()) as Box<dyn LogPlugin>),
            tx_prefix: "tx:".to_string(),
            version_lock_plugin: None,
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
            runtime_expr: RExprRuntime::new(),
            tx_manager: TxManager::new_arc(
                &option.tx_prefix,
                option.log_plugin.clone(),
                option.tx_lock_wait_timeout,
                option.tx_check_interval,
            ),
            page_plugin: option.page_plugin,
            sql_intercepts: option.sql_intercepts,
            logic_plugin: option.logic_plugin,
            log_plugin: option.log_plugin,
            runtime_py: PyRuntime {
                cache: Default::default(),
                generate: option.generate,
            },
            version_lock_plugin: None,
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
        w = w.set_formats(T::formats(&self.driver_type().unwrap()));
        return w;
    }

    /// link pool
    pub async fn link(&self, driver_url: &str) -> Result<(), Error> {
        return Ok(self.link_opt(driver_url, &DBPoolOptions::default()).await?);
    }

    /// link pool by DBPoolOptions
    /// for example:
    ///          let mut opt = PoolOptions::new();
    ///          opt.max_size = 20;
    ///          rb.link_opt("mysql://root:123456@localhost:3306/test", &opt).await.unwrap();
    pub async fn link_opt(
        &self,
        driver_url: &str,
        pool_options: &DBPoolOptions,
    ) -> Result<(), Error> {
        if driver_url.is_empty() {
            return Err(Error::from("[rbatis] link url is empty!"));
        }
        if self.pool.get().is_none() {
            let pool = DBPool::new_opt_str(driver_url, pool_options).await?;
            self.pool.get_or_init(|| {
                return pool;
            });
        }
        return Ok(());
    }

    /// link pool by DBConnectOption and DBPoolOptions
    /// for example:
    ///         let db_cfg=DBConnectOption::from("mysql://root:123456@localhost:3306/test")?;
    ///         rb.link_cfg(&db_cfg,PoolOptions::new());
    pub async fn link_cfg(
        &self,
        connect_option: &DBConnectOption,
        pool_options: &DBPoolOptions,
    ) -> Result<(), Error> {
        if self.pool.get().is_none() {
            let pool = DBPool::new_opt(connect_option, pool_options).await?;
            self.pool.get_or_init(|| {
                return pool;
            });
        }
        return Ok(());
    }

    pub fn set_log_plugin(&mut self, arg: impl LogPlugin + 'static) {
        self.log_plugin = Arc::new(Box::new(arg));
    }

    pub fn set_logic_plugin(&mut self, arg: Option<impl LogicDelete + 'static>) {
        match arg {
            Some(v) => {
                self.logic_plugin = Some(Box::new(v));
            }
            None => {
                self.logic_plugin = None;
            }
        }
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
        Ok(pool.driver_type)
    }

    pub async fn acquire(&self) -> Result<RBatisConnExecutor<'_>, Error> {
        let pool = self.get_pool()?;
        let conn = pool.acquire().await?;
        return Ok(RBatisConnExecutor {
            sql: "".to_string(),
            args: vec![],
            conn: conn,
            rb: &self
        });
    }

    pub async fn acquire_begin(&self) -> Result<RBatisTxExecutor<'_>, Error> {
        let pool = self.get_pool()?;
        let conn = pool.begin().await?;
        return Ok(RBatisTxExecutor {
            sql: "".to_string(),
            args: vec![],
            conn: conn,
            rb: &self
        });
    }







    /// begin tx,if TxGuard Drop, tx will be commit(when_drop_commit==true) or rollback(when_drop_commit==false)
    /// tx_id must be 'tx:'+id,this method default is 'tx:'+uuid
    /// for example:
    ///         let guard = RB.begin_tx_defer(true).await?;
    ///         let v: serde_json::Value = RB.fetch(&guard.tx_id, "select count(1) from biz_activity;").await?;
    ///
    #[deprecated]
    pub async fn begin_tx_defer(&self, when_drop_commit: bool) -> Result<TxGuard, Error> {
        let tx_id = self.begin_tx().await?;
        let guard = TxGuard::new(&tx_id, when_drop_commit, self.tx_manager.clone());
        return Ok(guard);
    }

    /// begin tx,for new conn,return (String(context_id/tx_id),u64)
    /// tx_id must be 'tx:'+id,this method default is 'tx:'+uuid
    ///
    /// for example:
    ///  let tx_id = rb.begin_tx().await.unwrap();
    ///  let v: serde_json::Value = rb.fetch(&tx_id, "select count(1) from biz_activity;").await.unwrap();
    ///
    #[deprecated]
    pub async fn begin_tx(&self) -> Result<String, Error> {
        let new_context_id = format!(
            "{}{}",
            &self.tx_manager.tx_prefix,
            Uuid::new_v4().to_string()
        );
        return Ok(self.begin(&new_context_id).await?);
    }

    /// begin tx,if TxGuard Drop, tx will be commit(when_drop_commit==true) or rollback(when_drop_commit==false)
    /// arg context_id must be 'tx:***'
    ///
    /// for example:
    ///         let context_id = "tx:1";
    ///         let tx_id = rb.begin_defer(context_id,true).await.unwrap();
    ///         let v: serde_json::Value = rb.fetch(&tx_id, "select count(1) from biz_activity;").await.unwrap();
    ///
    #[deprecated]
    pub async fn begin_defer(
        &self,
        context_id: &str,
        when_drop_commit: bool,
    ) -> Result<TxGuard, Error> {
        let tx_id = self.begin(context_id).await?;
        let guard = TxGuard::new(&tx_id, when_drop_commit, self.tx_manager.clone());
        return Ok(guard);
    }

    /// begin tx,for new conn,return <u64(tx num),Error>
    /// arg context_id must be 'tx:***'
    ///
    /// for example:
    ///         let context_id = "tx:1";
    ///         rb.begin(context_id).await.unwrap();
    ///         let v: serde_json::Value = rb.fetch(context_id, "select count(1) from biz_activity;").await.unwrap();
    ///         println!("{}", v.clone());
    ///         rb.commit(context_id).await.unwrap();
    ///
    #[deprecated]
    pub async fn begin(&self, context_id: &str) -> Result<String, Error> {
        if context_id.is_empty() {
            return Err(Error::from("[rbatis] context_id can not be empty"));
        }
        if !self.tx_manager.is_tx_id(context_id) {
            return Err(Error::from(format!(
                "[rbatis] context_id: {}  must be start with '{}', for example: {}{}",
                &self.tx_manager.tx_prefix, &self.tx_manager.tx_prefix, context_id, context_id
            )));
        }
        let result = self.tx_manager.begin(context_id, self.get_pool()?).await?;
        return Ok(result);
    }

    /// commit tx,and return conn,return <u64(tx num),Error>
    #[deprecated]
    pub async fn commit(&self, context_id: &str) -> Result<String, Error> {
        if context_id.is_empty() {
            return Err(Error::from("[rbatis] context_id can not be empty"));
        }
        if !self.tx_manager.is_tx_id(context_id) {
            return Err(Error::from(format!(
                "[rbatis] context_id: {} must be start with '{}', for example: {}{}",
                &self.tx_manager.tx_prefix, &self.tx_manager.tx_prefix, context_id, context_id
            )));
        }
        let result = self.tx_manager.commit(context_id).await?;
        return Ok(result);
    }

    /// rollback tx,and return conn,return <u64(tx num),Error>
    #[deprecated]
    pub async fn rollback(&self, context_id: &str) -> Result<String, Error> {
        if context_id.is_empty() {
            return Err(Error::from("[rbatis] context_id can not be empty"));
        }
        if !self.tx_manager.is_tx_id(context_id) {
            return Err(Error::from(format!(
                "[rbatis] context_id: {} must be start with '{}', for example: {}{}",
                &self.tx_manager.tx_prefix, &self.tx_manager.tx_prefix, context_id, context_id
            )));
        }
        let result = self.tx_manager.rollback(context_id).await?;
        return Ok(result);
    }


    /// is debug mode
    pub fn is_debug_mode(&self) -> bool {
        if cfg!(feature = "debug_mode") {
            return true;
        }
        return false;
    }
}
