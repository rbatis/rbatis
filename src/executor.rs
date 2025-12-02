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
use crate::intercept::{Intercept, ResultType};

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

#[derive(Clone)]
pub struct RBatisConnExecutor {
    pub id: i64,
    pub rb: RBatis,
    pub conn: Arc<Mutex<Box<dyn Connection>>>,
    pub intercepts: Arc<SyncVec<Arc<dyn Intercept>>>,
}

impl RBatisConnExecutor {
    pub fn new(id: i64, conn: Box<dyn Connection>, rb: RBatis) -> Self {
        Self {
            id: id,
            conn: Arc::new(Mutex::new(conn)),
            intercepts: rb.intercepts.clone(),
            rb: rb,
        }
    }

    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        let conn = Arc::into_inner(self.conn);
        match conn {
            Option::Some(conn) => {
                let v= Mutex::into_inner(conn);
                Some(v)
            },
            Option::None => None,   
        }
    }

    pub fn intercepts(&self)->&SyncVec<Arc<dyn Intercept>>{
        return &self.intercepts;
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
            for item in self.intercepts.iter() {
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
            for item in self.intercepts.iter() {
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
            for item in self.intercepts.iter() {
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
            for item in self.intercepts.iter() {
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
            let task_id = self.rb.task_id_generator.generate();
            let id = self.id;
            let rb = self.rb.clone();
            let intercepts = self.intercepts.clone();
            let conn = self.take_connection();
            let mut conn = conn.ok_or_else(|| Error::from("Failed to unwrap Arc"))?;
            conn.begin().await?;
            let mut conn_executor = RBatisConnExecutor::new(
                id,
                conn,
                rb,
            );
            conn_executor.intercepts = intercepts;
            Ok(RBatisTxExecutor::new(
                task_id,
                conn_executor,
            ))
        })
    }

    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.rollback().await?) })
    }

    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { Ok(self.conn.lock().await.commit().await?) })
    }
    

    /// get intercept from name
    /// the default name just like `let name = std::any::type_name::<LogInterceptor>()`
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let name = std::any::type_name::<MockIntercept>();
    ///  let intercept = rb.get_intercept_dyn(name);
    /// ```
    pub fn get_intercept_dyn(&self, name: &str) -> Option<&dyn Intercept> {
        for x in self.intercepts.iter() {
            if name == x.name() {
                return Some(x.as_ref());
            }
        }
        None
    }

    /// get intercept from name
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let intercept = rb.get_intercept::<MockIntercept>();
    /// ```
    pub fn get_intercept<T: Intercept>(&self) -> Option<&T> {
        let name = std::any::type_name::<T>();
        for item in self.intercepts.iter() {
            if name == item.name() {
                let v:Option<&T> = <dyn Any>::downcast_ref::<T>(item.as_ref());
                return v;
            }
        }
        None
    }

    /// how to ge name
    /// ```rust
    /// pub struct Intercept{}
    /// let name = std::any::type_name::<Intercept>();
    /// ```
    pub fn remove_intercept_dyn<T: Intercept>(&self, name: &str) -> Option<Arc<dyn Intercept>> {
        let mut index = 0;
        for item in self.intercepts.iter() {
            if item.name() == name {
                //this is safe
                return self.intercepts.remove(index);
            }
            index += 1;
        }
        None
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
    pub conn_executor: RBatisConnExecutor,
    /// please use tx.done()
    /// if tx call .commit() or .rollback() done = true.
    /// if tx not call .commit() or .rollback() done = false
    done: Arc<AtomicBool>,
}

impl Debug for RBatisTxExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutor")
            .field("tx_id", &self.tx_id)
            .field("conn_executor", &self.conn_executor)
            .field("done", &self.done)
            .finish()
    }
}

impl<'a> RBatisTxExecutor {
    pub fn new(tx_id: i64, conn_executor: RBatisConnExecutor) -> Self {
        RBatisTxExecutor {
            tx_id: tx_id,
            conn_executor: conn_executor,
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
            self.conn_executor.conn.lock().await.begin().await?;
            Ok(self)
        })
    }

    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn_executor.conn.lock().await.rollback().await?;
            self.done.store(true, Ordering::Relaxed);
            Ok(r)
        })
    }

    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            let r = self.conn_executor.conn.lock().await.commit().await?;
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

    pub fn get_intercept_dyn(&self, name: &str) -> Option<&dyn Intercept> {
        self.conn_executor.get_intercept_dyn(name)
    }

   
    pub fn get_intercept<T: Intercept>(&self) -> Option<&T> {
        self.conn_executor.get_intercept::<T>()
    }

    pub fn remove_intercept_dyn<T: Intercept>(&self, name: &str) -> Option<Arc<dyn Intercept>> {
        self.conn_executor.remove_intercept_dyn::<T>(name)
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
            for item in self.conn_executor.intercepts.iter() {
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
            let mut result = self.conn_executor.conn.lock().await.exec(&sql, args).await;
            for item in self.conn_executor.intercepts.iter() {
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
            for item in self.conn_executor.intercepts.iter() {
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
            let mut conn = self.conn_executor.conn.lock().await;
            let mut args_after = args.clone();
            let conn = conn.get_values(&sql, args);
            let mut result = conn.await;
            for item in self.conn_executor.intercepts.iter() {
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
        &self.conn_executor.rb
    }
}

impl RBatisTxExecutor {
    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        self.conn_executor.take_connection()
    }

    pub fn intercepts(&self)->&SyncVec<Arc<dyn Intercept>>{
        return &self.conn_executor.intercepts();
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

    pub fn get_intercept_dyn(&self, name: &str) -> Option<&dyn Intercept> {
        self.tx.get_intercept_dyn(name)
    }

   
    pub fn get_intercept<T: Intercept>(&self) -> Option<&T> {
        self.tx.get_intercept::<T>()
    }

    pub fn remove_intercept_dyn<T: Intercept>(&self, name: &str) -> Option<Arc<dyn Intercept>> {
        self.tx.remove_intercept_dyn::<T>(name)
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
