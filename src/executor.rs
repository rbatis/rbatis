use crate::decode::decode;
use crate::intercept::ResultType;
use crate::rbatis::RBatis;
use crate::snowflake::new_snowflake_id;
use crate::sql::tx::Tx;
use crate::Error;
use futures::Future;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult};
use rbs::Value;
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Formatter};
use rbdc::rt::tokio::sync::Mutex;

/// the rbatis's Executor. this trait impl with structs = RBatis,RBatisConnExecutor,RBatisTxExecutor,RBatisTxExecutorGuard
pub trait Executor: RBatisRef + Send + Sync {
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
}

pub trait RBatisRef: Send + Sync {
    fn rbatis_ref(&self) -> &RBatis;

    fn driver_type(&self) -> crate::Result<&str> {
        self.rbatis_ref().driver_type()
    }
}

impl RBatisRef for RBatis {
    fn rbatis_ref(&self) -> &RBatis {
        self
    }
}

pub struct RBatisConnExecutor {
    pub conn: Mutex<Box<dyn Connection>>,
    pub rb: RBatis,
}

unsafe impl Sync for RBatisConnExecutor {}

impl Debug for RBatisConnExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rb.fmt(f)
    }
}

impl RBatisConnExecutor {
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<ExecResult, Error> {
        let v = Executor::exec(self, sql, args).await?;
        Ok(v)
    }

    pub async fn query(&self, sql: &str, args: Vec<Value>) -> Result<Value, Error> {
        let v = Executor::query(self, sql, args).await?;
        Ok(v)
    }

    pub async fn query_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let v = Executor::query(self, sql, args).await?;
        Ok(decode(v)?)
    }
}

impl Executor for RBatisConnExecutor {
    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let rb_task_id = new_snowflake_id();
            let mut before_result = Err(Error::from(""));
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .before(
                        rb_task_id,
                        self as &dyn Executor,
                        &mut sql,
                        &mut args,
                        ResultType::Exec(&mut before_result),
                    )
                    .await?;
                if !next {
                    return before_result;
                }
            }
            let mut result = self.conn.lock().await.exec(&sql, args).await;
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .after(
                        rb_task_id,
                        self as &dyn Executor,
                        &mut sql,
                        &mut vec![],
                        ResultType::Exec(&mut result),
                    )
                    .await?;
                if !next {
                    return result;
                }
            }
            result
        })
    }

    fn query(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let rb_task_id = new_snowflake_id();
            let mut before_result = Err(Error::from(""));
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .before(
                        rb_task_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Query(&mut before_result),
                    )
                    .await?;
                if !next {
                    return before_result.map(|v| Value::from(v));
                }
            }
            let mut conn = self.conn.lock().await;
            let mut result = conn.get_values(&sql, args).await;
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .after(
                        rb_task_id,
                        self,
                        &mut sql,
                        &mut vec![],
                        ResultType::Query(&mut result),
                    )
                    .await?;
                if !next {
                    return result.map(|v| Value::from(v));
                }
            }
            Ok(Value::Array(result?))
        })
    }
}

impl RBatisRef for RBatisConnExecutor {
    fn rbatis_ref(&self) -> &RBatis {
        &self.rb
    }
}

impl RBatisConnExecutor {
    pub async fn begin(self) -> crate::Result<RBatisTxExecutor> {
        let tx = self.conn.into_inner().begin().await?;
        return Ok(RBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: Mutex::new(tx),
            rb: self.rb,
            done: false,
        });
    }
}

pub struct RBatisTxExecutor {
    pub tx_id: i64,
    pub conn: Mutex<Box<dyn Connection>>,
    pub rb: RBatis,
    pub done: bool,
}

unsafe impl Sync for RBatisTxExecutor {}

impl Debug for RBatisTxExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutor")
            .field("tx_id", &self.tx_id)
            .field("rb", &self.rb)
            .finish()
    }
}

impl<'a> RBatisTxExecutor {
    /// exec
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<ExecResult, Error> {
        let v = Executor::exec(self, sql, args).await?;
        Ok(v)
    }
    /// query value
    pub async fn query(&self, sql: &str, args: Vec<Value>) -> Result<Value, Error> {
        let v = Executor::query(self, sql, args).await?;
        Ok(v)
    }
    /// query and decode
    pub async fn query_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let v = Executor::query(self, sql, args).await?;
        Ok(decode(v)?)
    }
}

impl Executor for RBatisTxExecutor {
    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let mut before_result = Err(Error::from(""));
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .before(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Exec(&mut before_result),
                    )
                    .await?;
                if !next {
                    return before_result;
                }
            }
            let mut result = self.conn.lock().await.exec(&sql, args).await;
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .after(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut vec![],
                        ResultType::Exec(&mut result),
                    )
                    .await?;
                if !next {
                    return result;
                }
            }
            result
        })
    }

    fn query(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let mut before_result = Err(Error::from(""));
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .before(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Query(&mut before_result),
                    )
                    .await?;
                if !next {
                    return before_result.map(|v| Value::from(v));
                }
            }
            let mut conn = self.conn.lock().await;
            let conn = conn.get_values(&sql, args);
            let mut result = conn.await;
            for item in self.rbatis_ref().intercepts.iter() {
                let next = item
                    .after(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut vec![],
                        ResultType::Query(&mut result),
                    )
                    .await?;
                if !next {
                    return result.map(|v| Value::from(v));
                }
            }
            Ok(Value::Array(result?))
        })
    }
}

impl RBatisRef for RBatisTxExecutor {
    fn rbatis_ref(&self) -> &RBatis {
        &self.rb
    }
}

impl RBatisTxExecutor {
    pub async fn begin(mut self) -> crate::Result<Self> {
        self.conn = Mutex::new(self.conn.into_inner().begin().await?);
        return Ok(self);
    }
    pub async fn commit(&mut self) -> crate::Result<bool> {
        if let Ok(()) = self.conn.lock().await.commit().await {
            self.done = true;
        }
        return Ok(self.done);
    }
    pub async fn rollback(&mut self) -> crate::Result<bool> {
        if let Ok(()) = self.conn.lock().await.rollback().await {
            self.done = true;
        }
        return Ok(self.done);
    }

    pub fn take_conn(self) -> Option<Box<dyn Connection>> {
        return Some(self.conn.into_inner());
    }
}

pub struct RBatisTxExecutorGuard {
    pub tx: Option<RBatisTxExecutor>,
    pub callback: Box<dyn FnMut(RBatisTxExecutor) + Send>,
}

// unsafe impl Send for RBatisTxExecutorGuard{}
unsafe impl Sync for RBatisTxExecutorGuard {}

impl Debug for RBatisTxExecutorGuard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutorGuard")
            .field("tx", &self.tx)
            .finish()
    }
}

impl RBatisTxExecutorGuard {
    pub async fn begin(&mut self) -> crate::Result<()> {
        let v = self
            .tx
            .take()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?
            .begin()
            .await?;
        self.tx = Some(v);
        return Ok(());
    }

    pub async fn commit(&mut self) -> crate::Result<bool> {
        let tx = self
            .tx
            .as_mut()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.commit().await?);
    }

    pub async fn rollback(&mut self) -> crate::Result<bool> {
        let tx = self
            .tx
            .as_mut()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.rollback().await?);
    }

    pub fn take_conn(mut self) -> Option<Box<dyn Connection>> {
        match self.tx.take() {
            None => None,
            Some(s) => s.take_conn(),
        }
    }

    pub async fn query_decode<T>(&mut self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let tx = self
            .tx
            .as_mut()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        tx.query_decode(sql, args).await
    }
}

impl RBatisTxExecutor {
    /// defer and use future method
    /// for example:
    ///         tx.defer_async(|mut tx| async {
    ///             tx.rollback().await;
    ///         });
    ///
    pub fn defer_async<F>(self, callback: fn(s: RBatisTxExecutor) -> F) -> RBatisTxExecutorGuard
    where
        F: Future<Output = ()> + Send + 'static,
    {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(move |arg| {
                let rb = arg.rbatis_ref().clone();
                let future = callback(arg);
                if let Ok(pool) = rb.get_pool() {
                    pool.spawn_task(future);
                }
            }),
        }
    }
}

impl Drop for RBatisTxExecutorGuard {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            (self.callback)(tx);
        }
    }
}

impl RBatisRef for RBatisTxExecutorGuard {
    fn rbatis_ref(&self) -> &RBatis {
        self.tx.as_ref().unwrap().rbatis_ref()
    }
}

impl Executor for RBatisTxExecutorGuard {
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            match self.tx.as_ref() {
                None => Err(Error::from("the tx is done!")),
                Some(tx) => tx.exec(&sql, args).await,
            }
        })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            match self.tx.as_ref() {
                None => Err(Error::from("the tx is done!")),
                Some(tx) => tx.query(&sql, args).await,
            }
        })
    }
}

impl RBatis {
    /// exec sql
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<rbdc::db::ExecResult, Error> {
        let conn = self.acquire().await?;
        conn.exec(sql, args).await
    }

    /// query raw Value
    pub async fn query(&self, sql: &str, args: Vec<Value>) -> Result<Value, Error> {
        let conn = self.acquire().await?;
        let v = conn.query(sql, args).await?;
        Ok(v)
    }

    /// query and decode
    pub async fn query_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let conn = self.acquire().await?;
        let v = conn.query(sql, args).await?;
        Ok(decode(v)?)
    }
}

impl Executor for RBatis {
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let conn = self.acquire().await?;
            conn.exec(&sql, args).await
        })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let conn = self.acquire().await?;
            conn.query(&sql, args).await
        })
    }
}

impl RBatisRef for &RBatis {
    fn rbatis_ref(&self) -> &RBatis {
        self
    }
}

impl Executor for &RBatis {
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let conn = self.acquire().await?;
            conn.exec(&sql, args).await
        })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            let conn = self.acquire().await?;
            conn.query(&sql, args).await
        })
    }
}
