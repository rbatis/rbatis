use async_trait::async_trait;
use rbatis_core::db::DBExecResult;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::DriverType;

#[async_trait]
pub trait Executor {
    async fn exec_prepare(&mut self, sql: &str, args: &Vec<serde_json::Value>) -> Result<DBExecResult, Error>;
    async fn exec(&mut self, sql: &str) -> Result<DBExecResult, Error>;

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
pub struct RBatisConnExecutor {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub conn: DBPoolConn,
}

#[async_trait]
impl Executor for RBatisConnExecutor {
    async fn exec_prepare(&mut self, sql: &str, args: &Vec<Value>) -> Result<DBExecResult, Error> {
        let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
        let result = self.conn.exec_prepare(q).await;
        return result;
    }

    async fn exec(&mut self, sql: &str) -> Result<DBExecResult, Error> {
        self.conn.execute(sql).await
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub conn: DBTx,
}

#[async_trait]
impl Executor for RBatisTxExecutor {
    async fn exec_prepare(&mut self, sql: &str, args: &Vec<Value>) -> Result<DBExecResult, Error> {
        let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, &args)?;
        let result = self.conn.exec_prepare(q).await;
        return result;
    }

    async fn exec(&mut self, sql: &str) -> Result<DBExecResult, Error> {
        self.conn.execute(sql).await
    }
}
