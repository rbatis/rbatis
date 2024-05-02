use crate::decode::decode;
use crate::intercept::ResultType;
use crate::rbatis::RBatis;
use crate::snowflake::new_snowflake_id;
use crate::Error;
use dark_std::sync::SyncVec;
use futures::Future;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult};
use rbdc::rt::tokio::sync::Mutex;
use rbs::Value;
use serde::de::DeserializeOwned;
use std::fmt::{Debug, Formatter};

/// the rbatis's Executor. this trait impl with structs = RBatis,RBatisConnExecutor,RBatisTxExecutor,RBatisTxExecutorGuard
pub trait Executor: RBatisRef + Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
}

pub trait RBatisRef: Send + Sync {
    fn rb_ref(&self) -> &RBatis;

    fn driver_type(&self) -> crate::Result<&str> {
        self.rb_ref().driver_type()
    }
}

impl RBatisRef for RBatis {
    fn rb_ref(&self) -> &RBatis {
        self
    }
}

pub struct RBatisConnExecutor {
    pub id: i64,
    pub conn: Mutex<Box<dyn Connection>>,
    pub rb: RBatis,
}

impl Debug for RBatisConnExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutor")
            .field("id", &self.id)
            .field("rb", &self.rb)
            .finish()
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
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .before(
                        rb_task_id,
                        self as &dyn Executor,
                        &mut sql,
                        &mut args,
                        ResultType::Exec(&mut before_result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
                    return before_result;
                }
            }
            let mut args_after = args.clone();
            let mut result = self.conn.lock().await.exec(&sql, args).await;
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .after(
                        rb_task_id,
                        self as &dyn Executor,
                        &mut sql,
                        &mut args_after,
                        ResultType::Exec(&mut result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
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
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .before(
                        rb_task_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Query(&mut before_result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
                    return before_result.map(|v| Value::from(v));
                }
            }
            let mut conn = self.conn.lock().await;
            let mut args_after = args.clone();
            let mut result = conn.get_values(&sql, args).await;
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .after(
                        rb_task_id,
                        self,
                        &mut sql,
                        &mut args_after,
                        ResultType::Query(&mut result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
                    return result.map(|v| Value::from(v));
                }
            }
            Ok(Value::Array(result?))
        })
    }
}

impl RBatisRef for RBatisConnExecutor {
    fn rb_ref(&self) -> &RBatis {
        &self.rb
    }
}

impl RBatisConnExecutor {
    pub fn begin(self) -> BoxFuture<'static, Result<RBatisTxExecutor, Error>> {
        Box::pin(async move {
            let mut conn = self.conn.into_inner();
            conn.begin().await?;
            Ok(RBatisTxExecutor::new(self.rb, conn))
        })
    }

    pub fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.rollback().await?) })
    }

    pub fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.commit().await?) })
    }
}

pub struct RBatisTxExecutor {
    pub tx_id: i64,
    pub conn: Mutex<Box<dyn Connection>>,
    pub rb: RBatis,
    /// if tx call .commit() or .rollback() done = true.
    /// if tx not call .commit() or .rollback() done = false
    pub done: bool,
}

impl Debug for RBatisTxExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutor")
            .field("tx_id", &self.tx_id)
            .field("rb", &self.rb)
            .field("done", &self.done)
            .finish()
    }
}

impl<'a> RBatisTxExecutor {
    pub fn new(rb: RBatis, conn: Box<dyn Connection>) -> Self {
        RBatisTxExecutor {
            tx_id: 0,
            conn: Mutex::new(conn),
            rb: rb,
            done: false,
        }
    }

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

    pub fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.conn.lock().await.begin().await?;
            Ok(self)
        })
    }

    pub fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn.lock().await.rollback().await?;
            self.done = true;
            Ok(r)
        })
    }

    pub fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn.lock().await.commit().await?;
            self.done = true;
            Ok(r)
        })
    }
}

impl Executor for RBatisTxExecutor {
    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let mut before_result = Err(Error::from(""));
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .before(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Exec(&mut before_result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
                    return before_result;
                }
            }
            let mut args_after = args.clone();
            let mut result = self.conn.lock().await.exec(&sql, args).await;
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .after(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args_after,
                        ResultType::Exec(&mut result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
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
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .before(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args,
                        ResultType::Query(&mut before_result),
                    )
                    .await?;
                if let Some(next) = next {
                    if next {
                        break;
                    }
                } else {
                    return before_result.map(|v| Value::from(v));
                }
            }
            let mut conn = self.conn.lock().await;
            let mut args_after = args.clone();
            let conn = conn.get_values(&sql, args);
            let mut result = conn.await;
            for item in self.rb_ref().intercepts.iter() {
                let next = item
                    .after(
                        self.tx_id,
                        self,
                        &mut sql,
                        &mut args_after,
                        ResultType::Query(&mut result),
                    )
                    .await?;
                if let Some(next) = next {
                    if !next {
                        break;
                    }
                } else {
                    return result.map(|v| Value::from(v));
                }
            }
            Ok(Value::Array(result?))
        })
    }
}

impl RBatisRef for RBatisTxExecutor {
    fn rb_ref(&self) -> &RBatis {
        &self.rb
    }
}

impl RBatisTxExecutor {
    pub fn take_conn(self) -> Option<Box<dyn Connection>> {
        return Some(self.conn.into_inner());
    }
}

pub struct RBatisTxExecutorGuard {
    pub tx: Option<RBatisTxExecutor>,
    pub callback: Box<dyn FnMut(RBatisTxExecutor) + Send + Sync>,
}

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

    pub async fn commit(&mut self) -> crate::Result<()> {
        let tx = self
            .tx
            .as_mut()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        tx.commit().await?;
        return Ok(());
    }

    pub async fn rollback(&mut self) -> crate::Result<()> {
        let tx = self
            .tx
            .as_mut()
            .ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        tx.rollback().await?;
        return Ok(());
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
                let future = callback(arg);
                rbdc::rt::spawn(async move {
                    future.await;
                });
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
    fn rb_ref(&self) -> &RBatis {
        self.tx.as_ref().unwrap().rb_ref()
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
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<ExecResult, Error> {
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
    fn rb_ref(&self) -> &RBatis {
        self
    }
}

#[derive(Debug)]
pub struct TempExecutor<'a> {
    pub rb: &'a RBatis,
    pub sql: SyncVec<String>,
    pub args: SyncVec<Vec<Value>>,
}

impl<'a> TempExecutor<'a> {
    pub fn new(rb: &'a RBatis) -> Self {
        Self {
            rb: rb,
            sql: SyncVec::new(),
            args: SyncVec::new(),
        }
    }

    pub fn clear_sql(&self) -> Vec<String> {
        let mut arr = vec![];
        loop {
            if let Some(v) = self.sql.remove(0) {
                arr.push(v);
            } else {
                break;
            }
        }
        arr
    }

    pub fn clear_args(&self) -> Vec<Vec<Value>> {
        let mut arr = vec![];
        loop {
            if let Some(v) = self.args.remove(0) {
                arr.push(v);
            } else {
                break;
            }
        }
        arr
    }
}

impl RBatisRef for TempExecutor<'_> {
    fn rb_ref(&self) -> &RBatis {
        self.rb
    }
}

impl Executor for TempExecutor<'_> {
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        self.sql.push(sql.to_string());
        self.args.push(args);
        Box::pin(async { Ok(ExecResult::default()) })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        self.sql.push(sql.to_string());
        self.args.push(args);
        Box::pin(async { Ok(Value::default()) })
    }
}
