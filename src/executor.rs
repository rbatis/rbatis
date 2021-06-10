use async_trait::async_trait;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::DriverType;
use crate::rbatis::Rbatis;
use std::ops::Deref;
use crate::utils::string_util;
use futures::Future;


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
                "",
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
                        "",
                        &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.log_plugin
                        .error("", &format!("ReturnErr  <== {}", e));
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
                "",
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
                            .info("", &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.log_plugin
                            .error("", &format!("ReturnErr  <== {}", e));
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
                            .info("", &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.log_plugin
                            .error("", &format!("ReturnErr  <== {}", e));
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
    pub async fn commit(mut self) -> crate::Result<()> {
        self.conn.commit().await
    }
    pub async fn rollback(mut self) -> crate::Result<()> {
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
    pub callback: fn(s:RBatisTxExecutor<'a>),
}

impl <'a>RBatisTxExecutor<'a> {
    pub fn to_defer(self,callback: fn(s:Self))->RBatisTxExecutorGuard<'a>{
        RBatisTxExecutorGuard{
            tx: Some(self),
            callback: callback
        }
    }
}


impl Drop for RBatisTxExecutorGuard<'_>{
    fn drop(&mut self) {
        match self.tx.take(){
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