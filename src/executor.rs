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

#[async_trait]
pub trait Executor {
    fn get_rbatis(&self) -> &Rbatis;

    fn driver_type(&self) -> crate::Result<DriverType> {
        self.get_rbatis().driver_type()
    }

    async fn execute(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned;

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

#[derive(Debug)]
pub struct RBatisConnExecutor<'a> {
    pub conn: DBPoolConn,
    pub rb: &'a Rbatis,
}

macro_rules! impl_executor {
    ($t:ty) => {
#[async_trait]
impl<'a> Executor for $t {

    fn get_rbatis(&self)-> &Rbatis{
        return &self.rb;
    }

    async fn execute(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error> {
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
                    "Exec  ==> {}\n{}[rbatis] [{}] Args  ==> {}",
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
                    "Exec  ==> {}\n{}[rbatis] [{}] Args  ==> {}",
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
    };
}

impl_executor!(RBatisConnExecutor<'a>);


impl <'a>RBatisConnExecutor<'a> {
    pub async fn begin(&'a mut self) -> crate::Result<RBatisTxExecutor<'a>> {
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
}

impl_executor!(RBatisTxExecutor<'a>);


impl<'a> RBatisTxExecutor<'a> {
    pub async fn commit(&mut self) -> crate::Result<()> {
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