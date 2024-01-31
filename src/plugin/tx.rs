use async_trait::async_trait;
use crate::Error;
use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbs::Value;
use crate::executor::{Executor, RBatisConnExecutor, RBatisTxExecutor};
use crate::intercept::{Intercept, ResultType};

pub trait Tx {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>>
        where
            Self: Sized;
    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>>;
    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>>;
}

#[derive(Debug)]
pub struct TxIntercept {
    //begin (sql,args)
    pub begin: (String, Vec<Value>),
    //commit (sql,args)
    pub commit: (String, Vec<Value>),
    //rollback (sql,args)
    pub rollback: (String, Vec<Value>),
}

impl TxIntercept {
    pub fn new() -> Self {
        Self {
            begin: ("begin".to_string(), vec![]),
            commit: ("commit".to_string(), vec![]),
            rollback: ("rollback".to_string(), vec![]),
        }
    }
}

#[async_trait]
impl Intercept for TxIntercept {
    async fn before(&self, _task_id: i64, _rb: &dyn Executor, sql: &mut String, args: &mut Vec<Value>, _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>) -> Result<bool, Error> {
        if sql == "begin" {
            *sql = self.begin.0.clone();
            *args = self.begin.1.clone();
        } else if sql == "commit" {
            *sql = self.commit.0.clone();
            *args = self.commit.1.clone();
        } else if sql == "rollback" {
            *sql = self.rollback.0.clone();
            *args = self.rollback.1.clone();
        }
        Ok(true)
    }
}


impl Tx for RBatisTxExecutor {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.exec("begin", vec![]).await?;
            Ok(self)
        })
    }

    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("rollback", vec![]).await?;
            Ok(())
        })
    }

    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("commit", vec![]).await?;
            Ok(())
        })
    }
}


impl Tx for RBatisConnExecutor {
    fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.exec("begin", vec![]).await?;
            Ok(self)
        })
    }

    fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("rollback", vec![]).await?;
            Ok(())
        })
    }

    fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.exec("commit", vec![]).await?;
            Ok(())
        })
    }
}