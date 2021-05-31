use async_trait::async_trait;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::DriverType;
use crate::rbatis::Rbatis;
use std::ops::Deref;

#[async_trait]
pub trait Executor {
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

#[async_trait]
impl<'a> Executor for RBatisConnExecutor<'a> {
    async fn execute(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            let result = self.conn.exec_prepare(q).await;
            return result;
        } else {
            let result = self.conn.execute(&sql).await;
            return result;
        }
    }

    async fn fetch<T>(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            let result: (T, usize) = self.conn.fetch_parperd(q).await?;
            return Ok(result.0);
        } else {
            let result: (T, usize) = self.conn.fetch(&sql).await?;
            return Ok(result.0);
        }
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor<'a> {
    pub conn: DBTx,
    pub rb: &'a Rbatis,
}

#[async_trait]
impl<'a> Executor for RBatisTxExecutor<'a> {
    async fn execute(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            let result = self.conn.exec_prepare(q).await;
            return result;
        } else {
            let result = self.conn.execute(&sql).await;
            return result;
        }
    }

    async fn fetch<T>(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<T, Error> where T: DeserializeOwned {
        let mut sql = sql.to_string();
        let mut args = args.clone();
        let is_prepared = args.len() > 0;
        for item in &self.sql_intercepts {
            item.do_intercept(self, &mut sql, &mut args, is_prepared)?;
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
            let result: (T, usize) = self.conn.fetch_parperd(q).await?;
            return Ok(result.0);
        } else {
            let result: (T, usize) = self.conn.fetch(&sql).await?;
            return Ok(result.0);
        }
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