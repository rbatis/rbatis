use std::ops::Deref;

use async_trait::async_trait;
use futures::Future;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::crud::CRUD;
use crate::DriverType;
use crate::plugin::page::{IPageRequest, Page};
use crate::py::PySqlConvert;
use crate::rbatis::Rbatis;
use crate::utils::string_util;

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
            q.bind_value(x);
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

macro_rules! impl_executor {
    ($t:ty) => {
#[async_trait]
impl<'a> ExecutorMut for $t {
    async fn exec(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin.info(
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
            result = self.conn.execute(&sql).await;
        }
        if self.log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.log_plugin.info(
                        &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.log_plugin
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
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if self.log_plugin.is_enable() {
            self.log_plugin.info(
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
            if self.log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.log_plugin
                            .info(&format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.log_plugin
                            .error(&format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.log_plugin
                            .info(&format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.log_plugin
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

impl RBatisConnExecutor<'_> {
    pub async fn begin(&mut self) -> crate::Result<RBatisTxExecutor<'_>> {
        let tx = self.conn.begin().await?;
        return Ok(RBatisTxExecutor {
            conn: tx,
            rb: self.rb,
        });
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor<'a> {
    pub conn: DBTx<'a>,
    pub rb: &'a Rbatis,
    // pub callback: fn(s:Self),
}

impl_executor!(RBatisTxExecutor<'_>);

impl<'a> RBatisTxExecutor<'a> {
    pub async fn commit(self) -> crate::Result<()> {
        self.conn.commit().await
    }
    pub async fn rollback(self) -> crate::Result<()> {
        self.conn.rollback().await
    }
}


/// impl Deref has all the capabilities of RBatis
impl<'a> Deref for RBatisConnExecutor<'a> {
    type Target = Rbatis;
    fn deref(&self) -> &Self::Target {
        &self.rb
    }
}

/// impl Deref has all the capabilities of RBatis
impl<'a> Deref for RBatisTxExecutor<'a> {
    type Target = Rbatis;
    fn deref(&self) -> &Self::Target {
        &self.rb
    }
}


pub struct RBatisTxExecutorGuard<'a> {
    pub tx: Option<RBatisTxExecutor<'a>>,
    pub callback: fn(s: RBatisTxExecutor<'a>),
}

impl<'a> RBatisTxExecutor<'a> {
    pub fn defer(self, callback: fn(s: Self)) -> RBatisTxExecutorGuard<'a> {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: callback,
        }
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

/// impl Deref has all the capabilities of RBatis
impl<'a> Deref for RBatisTxExecutorGuard<'a> {
    type Target = Rbatis;
    fn deref(&self) -> &Self::Target {
        &self.tx.as_ref().unwrap()
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
    /// py str into py ast,run get sql,arg result
    pub fn py_to_sql<Arg>(
        &self,
        py_sql: &str,
        arg: &Arg,
    ) -> Result<(String, Vec<serde_json::Value>), Error>
        where
            Arg: Serialize + Send + Sync,
    {
        if self.rb.is_some() {
            return self.rb.as_ref().unwrap().py_to_sql(py_sql, arg);
        } else if self.conn.is_some() {
            return self.conn.as_ref().unwrap().py_to_sql(py_sql, arg);
        } else if self.tx.is_some() {
            return self.tx.as_ref().unwrap().py_to_sql(py_sql, arg);
        } else if self.guard.is_some() {
            return self.guard.as_ref().unwrap().py_to_sql(py_sql, arg);
        }
        return Err(Error::from("[rbatis] executor must have an value!"));
    }

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
            return self.guard.as_ref().unwrap().exec(sql, args).await;
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
            return self.guard.as_ref().unwrap().fetch(sql, args).await;
        }
        return Err(Error::from("[rbatis] executor must have an value!"));
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