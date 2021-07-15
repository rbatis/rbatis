use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use futures::Future;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::crud::{CRUD, CRUDMut};
use crate::DriverType;
use crate::plugin::page::{IPageRequest, Page};
use crate::rbatis::Rbatis;
use crate::utils::string_util;
use futures::executor::block_on;


#[async_trait]
pub trait RbatisRef {
    fn get_rbatis(&self) -> &Rbatis;

    fn driver_type(&self) -> crate::Result<DriverType> {
        self.get_rbatis().driver_type()
    }

    /// bind arg into DBQuery
    fn bind_arg<'arg>(
        &self,
        driver_type: &DriverType,
        sql: &'arg str,
        arg: &Vec<serde_json::Value>,
    ) -> Result<DBQuery<'arg>, Error> {
        let mut q: DBQuery = DBPool::make_db_query(driver_type, sql)?;
        for x in arg {
            (self.get_rbatis().encoder)(&mut q, x)?;
        }
        return Ok(q);
    }
}

impl RbatisRef for Rbatis {
    fn get_rbatis(&self) -> &Rbatis {
        &self
    }
}

#[async_trait]
pub trait ExecutorMut: RbatisRef {
    async fn exec(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned;
}

#[derive(Debug)]
pub struct RBatisConnExecutor<'a> {
    pub conn: DBPoolConn,
    pub rb: &'a Rbatis,
}

impl<'a> RBatisConnExecutor<'a> {
    pub fn as_executor(&'a mut self) -> RbatisExecutor<'a> {
        self.into()
    }
}

macro_rules! impl_executor {
    ($t:ty) => {
#[async_trait]
impl<'a> ExecutorMut for $t {
    async fn exec(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            self.get_rbatis().log_plugin.info(
                &format!(
                    "Exec   ==> {}\n{}[rbatis] [{}] Args   ==> {}",
                    &sql,
                    string_util::LOG_SPACE,
                    "",
                    serde_json::Value::Array(args.clone()).to_string()
                ),
            );
        }
        let result;
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            result = self.conn.exec_prepare(q).await;
        } else {
            result = self.conn.exec(&sql).await;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.get_rbatis().log_plugin.info(
                        &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.get_rbatis().log_plugin
                        .error(&format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    async fn fetch<T>(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            self.get_rbatis().log_plugin.info(
                &format!(
                    "Fetch  ==> {}\n{}[rbatis] [{}] Args   ==> {}",
                    &sql,
                    string_util::LOG_SPACE,
                    "",
                    serde_json::Value::Array(args.clone()).to_string()
                ),
            );
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            let result = self.conn.fetch_parperd(q).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(&format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(&format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(&format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(&format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        }
    }
}

impl RbatisRef for $t {
    fn get_rbatis(&self) -> &Rbatis {
    self.rb
    }
}

};
}

impl_executor!(RBatisConnExecutor<'_>);

impl<'a> RBatisConnExecutor<'a> {
    pub async fn begin(self) -> crate::Result<RBatisTxExecutor<'a>> {
        let tx = self.conn.begin().await?;
        return Ok(RBatisTxExecutor {
            conn: tx,
            rb: self.rb,
        });
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor<'a> {
    pub conn: DBTx,
    pub rb: &'a Rbatis,
}

impl<'a> RBatisTxExecutor<'a> {
    pub fn as_executor(&'a mut self) -> RbatisExecutor<'a> {
        self.into()
    }
}

impl_executor!(RBatisTxExecutor<'_>);

impl<'a> RBatisTxExecutor<'a> {
    pub async fn begin(&mut self) -> crate::Result<()> {
        return Ok(self.conn.begin().await?);
    }
    pub async fn commit(&mut self) -> crate::Result<()> {
        return Ok(self.conn.commit().await?);
    }
    pub async fn rollback(&mut self) -> crate::Result<()> {
        return Ok(self.conn.rollback().await?);
    }

    pub fn take_conn(self) -> Option<DBPoolConn> {
        return self.conn.take_conn();
    }
}

impl<'a> Deref for RBatisTxExecutor<'a> {
    type Target = DBTx;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl<'a> DerefMut for RBatisTxExecutor<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}


pub struct RBatisTxExecutorGuard<'a> {
    pub tx: Option<RBatisTxExecutor<'a>>,
    pub callback: Box<dyn FnMut(RBatisTxExecutor<'a>) + Send + 'a>,
}

impl<'a> RBatisTxExecutorGuard<'a> {
    pub fn as_executor(&'a mut self) -> RbatisExecutor<'a> {
        self.into()
    }

    pub async fn begin(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.begin().await?);
    }

    pub async fn commit(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.commit().await?);
    }

    pub async fn rollback(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.rollback().await?);
    }

    pub fn take_conn(mut self) -> Option<DBPoolConn> {
        match self.tx.take() {
            None => {
                None
            }
            Some(s) => {
                s.take_conn()
            }
        }
    }
}

impl<'a> RBatisTxExecutor<'a> {
    /// defer an func
    /// for example:
    ///     tx.defer(|tx| {});
    ///
    pub fn defer<Call>(self, callback: Call) -> RBatisTxExecutorGuard<'a>
        where Call: 'a + FnMut(Self) + Send {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(callback),
        }
    }

    /// defer and use future method
    /// for example:
    ///         tx.defer_async(|tx| async {
    ///             tx.rollback().await;
    ///         });
    ///
    pub fn defer_async<R, F>(self, mut callback: F) -> RBatisTxExecutorGuard<'a>
        where R: 'a + Future<Output=()>,
              F: 'a + Send + FnMut(RBatisTxExecutor<'a>) -> R {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(move |arg| {
                block_on(callback(arg));
            }),
        }
    }

    pub async fn fetch_page<T>(
        &self,
        sql: &str,
        args: &Vec<serde_json::Value>,
        page_request: &dyn IPageRequest,
    ) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
    {
        self.get_rbatis().fetch_page(sql, args, page_request).await
    }
}

impl<'a> Deref for RBatisTxExecutorGuard<'a> {
    type Target = RBatisTxExecutor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.tx.as_ref().unwrap()
    }
}

impl<'a> DerefMut for RBatisTxExecutorGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx.as_mut().unwrap()
    }
}


impl Drop for RBatisTxExecutorGuard<'_> {
    fn drop(&mut self) {
        match self.tx.take() {
            None => {}
            Some(tx) => {
                (self.callback)(tx);
            }
        }
    }
}


#[async_trait]
pub trait Executor: RbatisRef {
    async fn exec(&self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned;
}

#[async_trait]
impl Executor for Rbatis {
    async fn exec(&self, sql: &str, args: &Vec<Value>) -> Result<DBExecResult, Error> {
        self.acquire().await?.exec(sql, args).await
    }

    async fn fetch<T>(&self, sql: &str, args: &Vec<Value>) -> Result<T, Error> where T: DeserializeOwned {
        self.acquire().await?.fetch(sql, args).await
    }
}

/// must be only one have Some(Value)
/// (&rb).into()
/// (&mut tx).into()
/// (&mut conn).into()
/// (&mut guard).into()
pub struct RbatisExecutor<'a> {
    pub rb: Option<&'a Rbatis>,
    pub conn: Option<&'a mut RBatisConnExecutor<'a>>,
    pub tx: Option<&'a mut RBatisTxExecutor<'a>>,
    pub guard: Option<&'a mut RBatisTxExecutorGuard<'a>>,
}

impl RbatisExecutor<'_> {
    pub async fn fetch_page<T>(&mut self, sql: &str, args: &Vec<Value>, page_request: &dyn IPageRequest) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync {
        if self.rb.is_some() {
            return self.rb.as_ref().unwrap().fetch_page(sql, args, page_request).await;
        } else if self.conn.is_some() {
            return self.conn.as_deref_mut().unwrap().fetch_page(sql, args, page_request).await;
        } else if self.tx.is_some() {
            return self.tx.as_deref_mut().unwrap().fetch_page(sql, args, page_request).await;
        } else if self.guard.is_some() {
            return self.guard.as_ref().unwrap().fetch_page(sql, args, page_request).await;
        }
        return Err(Error::from("[rbatis] executor must have an value!"));
    }

    pub async fn exec(&mut self, sql: &str, args: &Vec<Value>) -> Result<DBExecResult, Error> {
        if self.rb.is_some() {
            return self.rb.as_ref().unwrap().exec(sql, args).await;
        } else if self.conn.is_some() {
            return self.conn.as_deref_mut().unwrap().exec(sql, args).await;
        } else if self.tx.is_some() {
            return self.tx.as_deref_mut().unwrap().exec(sql, args).await;
        } else if self.guard.is_some() {
            return self.guard.as_deref_mut().unwrap().exec(sql, args).await;
        }
        return Err(Error::from("[rbatis] executor must have an value!"));
    }

    pub async fn fetch<T>(&mut self, sql: &str, args: &Vec<Value>) -> Result<T, Error> where T: DeserializeOwned {
        if self.rb.is_some() {
            return self.rb.as_ref().unwrap().fetch(sql, args).await;
        } else if self.conn.is_some() {
            return self.conn.as_deref_mut().unwrap().fetch(sql, args).await;
        } else if self.tx.is_some() {
            return self.tx.as_deref_mut().unwrap().fetch(sql, args).await;
        } else if self.guard.is_some() {
            return self.guard.as_deref_mut().unwrap().fetch(sql, args).await;
        }
        return Err(Error::from("[rbatis] executor must have an value!"));
    }
}

impl<'a> RbatisRef for RbatisExecutor<'a> {
    fn get_rbatis(&self) -> &Rbatis {
        if self.rb.is_some() {
            return self.rb.as_ref().unwrap();
        } else if self.tx.is_some() {
            return self.tx.as_ref().unwrap().get_rbatis();
        } else if self.conn.is_some() {
            return self.conn.as_ref().unwrap().get_rbatis();
        } else if self.guard.is_some() {
            return self.guard.as_ref().unwrap().get_rbatis();
        } else {
            panic!("[rbatis] executor must have one Some value!");
        }
    }
}

impl<'a> From<&'a Rbatis> for RbatisExecutor<'a> {
    fn from(arg: &'a Rbatis) -> Self {
        Self {
            rb: Some(arg),
            conn: None,
            tx: None,
            guard: None,
        }
    }
}

impl<'a> From<&'a mut RBatisConnExecutor<'a>> for RbatisExecutor<'a> {
    fn from(arg: &'a mut RBatisConnExecutor<'a>) -> Self {
        Self {
            rb: None,
            conn: Some(arg),
            tx: None,
            guard: None,
        }
    }
}

impl<'a> From<&'a mut RBatisTxExecutor<'a>> for RbatisExecutor<'a> {
    fn from(arg: &'a mut RBatisTxExecutor<'a>) -> Self {
        Self {
            rb: None,
            conn: None,
            tx: Some(arg),
            guard: None,
        }
    }
}

impl<'a> From<&'a mut RBatisTxExecutorGuard<'a>> for RbatisExecutor<'a> {
    fn from(arg: &'a mut RBatisTxExecutorGuard<'a>) -> Self {
        Self {
            rb: None,
            conn: None,
            tx: None,
            guard: Some(arg),
        }
    }
}