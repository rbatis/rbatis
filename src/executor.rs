use crate::decode::decode;
use crate::rbatis::RBatis;
use crate::Error;
use dark_std::sync::SyncVec;
use futures::Future;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult};
use rbdc::rt::tokio::sync::Mutex;
use rbs::Value;
use serde::de::DeserializeOwned;
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::intercept::ResultType;

/// the RBatis Executor. this trait impl with structs = RBatis,RBatisConnExecutor,RBatisTxExecutor,RBatisTxExecutorGuard
pub trait Executor: RBatisRef + Send + Sync {
    fn id(&self) -> i64;
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}

pub trait RBatisRef: Any + Send + Sync {
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
    pub rb: RBatis,
    pub conn: Mutex<Box<dyn Connection>>,
}

impl RBatisConnExecutor {
    pub fn new(id: i64, conn: Box<dyn Connection>, rb: RBatis) -> Self {
        Self {
            id: id,
            conn: Mutex::new(conn),
            rb: rb,
        }
    }
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
    fn id(&self) -> i64 {
        self.id
    }

    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let rb_task_id = self.rb.task_id_generator.generate();
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
            let rb_task_id = self.rb.task_id_generator.generate();
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
            Ok(RBatisTxExecutor::new(
                self.rb.task_id_generator.generate(),
                self.rb,
                conn,
            ))
        })
    }

    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.rollback().await?) })
    }

    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.commit().await?) })
    }
}

/// `RBatisTxExecutor` is a type that represents an executor for transactional operations in RBatis.
///
/// # Type Description
///
/// The `RBatisTxExecutor` is responsible for executing SQL statements within the context of a transaction.
/// It provides methods to execute queries, updates, and other SQL operations, ensuring that all operations
/// are part of the same transactional context.
#[derive(Clone)]
pub struct RBatisTxExecutor {
    pub tx_id: i64,
    pub conn: Arc<Mutex<Box<dyn Connection>>>,
    pub rb: RBatis,
    /// please use tx.done()
    /// if tx call .commit() or .rollback() done = true.
    /// if tx not call .commit() or .rollback() done = false
    done: Arc<AtomicBool>,
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
    pub fn new(tx_id: i64, rb: RBatis, conn: Box<dyn Connection>) -> Self {
        RBatisTxExecutor {
            tx_id: tx_id,
            conn: Arc::new(Mutex::new(conn)),
            rb: rb,
            done: Arc::new(AtomicBool::new(false)),
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

    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn.lock().await.rollback().await?;
            self.done.store(true, Ordering::Relaxed);
            Ok(r)
        })
    }

    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn.lock().await.commit().await?;
            self.done.store(true, Ordering::Relaxed);
            Ok(r)
        })
    }

    /// tx is done?
    pub fn done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    ///change is done
    pub fn set_done(&self, done: bool) {
        self.done.store(done, Ordering::Relaxed)
    }
}

impl Executor for RBatisTxExecutor {
    fn id(&self) -> i64 {
        self.tx_id
    }

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
                    if !next {
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
    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        match Arc::into_inner(self.conn) {
            None => None,
            Some(v) => {
                let inner = v.into_inner();
                Some(inner)
            }
        }
    }
}

/// `RBatisTxExecutorGuard` is a guard object that manages transactions for RBatis.
///
/// # Type Description
///
/// The `RBatisTxExecutorGuard` implements the `Drop` trait to ensure that transactions are
/// automatically committed or rolled back when the guard goes out of scope. It encapsulates
/// the transaction executor and provides a set of methods to manipulate database transactions.
#[derive(Clone)]
pub struct RBatisTxExecutorGuard {
    pub tx: RBatisTxExecutor,
    pub callback: Arc<dyn FnMut(RBatisTxExecutor) + Send + Sync>,
}

impl Debug for RBatisTxExecutorGuard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutorGuard")
            .field("tx", &self.tx)
            .finish()
    }
}

impl RBatisTxExecutorGuard {
    pub fn tx_id(&self) -> i64 {
        self.tx.tx_id
    }

    pub async fn commit(&self) -> crate::Result<()> {
        self.tx.commit().await?;
        Ok(())
    }

    pub async fn rollback(&self) -> crate::Result<()> {
        self.tx.rollback().await?;
        Ok(())
    }

    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        self.tx.clone().take_connection()
    }

    pub async fn query_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.tx.query_decode(sql, args).await
    }
}

impl RBatisTxExecutor {
    /// defer and use future method
    /// for example:
    /// ```rust
    ///  use rbatis::executor::RBatisTxExecutor;
    ///  use rbatis::{Error, RBatis};
    ///
    ///  async fn test_tx(tx:RBatisTxExecutor) -> Result<(),Error>{
    ///         tx.defer_async(|tx| async move {
    ///              if !tx.done(){ let _ = tx.rollback().await; }
    ///         });
    ///     Ok(())
    /// }
    /// ```
    pub fn defer_async<F>(&self, callback: fn(s: RBatisTxExecutor) -> F) -> RBatisTxExecutorGuard
    where
        F: Future<Output = ()> + Send + 'static,
    {
        RBatisTxExecutorGuard {
            tx: self.clone(),
            callback: Arc::new(move |arg| {
                let future = callback(arg);
                rbdc::rt::spawn(future);
            }),
        }
    }
}

impl Drop for RBatisTxExecutorGuard {
    fn drop(&mut self) {
        match Arc::get_mut(&mut self.callback) {
            None => {}
            Some(callback) => {
                callback(self.tx.clone());
            }
        }
    }
}

impl RBatisRef for RBatisTxExecutorGuard {
    fn rb_ref(&self) -> &RBatis {
        self.tx.rb_ref()
    }
}

impl Executor for RBatisTxExecutorGuard {
    fn id(&self) -> i64 {
        self.tx.id()
    }

    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let sql = sql.to_string();
        Box::pin(async move { self.tx.exec(&sql, args).await })
    }

    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let sql = sql.to_string();
        Box::pin(async move { self.tx.query(&sql, args).await })
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
    fn id(&self) -> i64 {
        0
    }

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

impl RBatisRef for TempExecutor<'static> {
    fn rb_ref(&self) -> &RBatis {
        self.rb
    }
}

impl Executor for TempExecutor<'static> {
    fn id(&self) -> i64 {
        0
    }

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
