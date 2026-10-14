//! Everything that can run SQL.
//!
//! [`Executor`] is the central trait of rbatis: the `crud!` macro, `#[py_sql]`,
//! `#[html_sql]` and `#[sql]` all accept a `&dyn Executor`, so the same generated
//! statement can run on the engine itself, on a pooled connection or inside a
//! transaction.
//!
//! | Type | Obtained from | Represents |
//! |------|---------------|------------|
//! | [`RBatis`] | [`RBatis::new()`] | the engine; every call borrows a connection from the pool for the duration of the call |
//! | [`RBatisConnExecutor`] | [`RBatis::acquire()`](crate::RBatis::acquire) | one checked out connection, reused by all statements run on it |
//! | [`RBatisTxExecutor`] | [`RBatis::acquire_begin()`](crate::RBatis::acquire_begin) | an open transaction (`BEGIN` already sent) |
//! | [`RBatisTxExecutorGuard`] | [`RBatisTxExecutor::defer_async`] / [`RBatisTxExecutor::auto_commit`] | a transaction that is committed or rolled back when the guard is dropped |
//!
//! # Running raw SQL
//!
//! ```rust,no_run
//! use rbatis::{Error, RBatis};
//! use rbs::to_value;
//!
//! #[derive(Debug, serde::Serialize, serde::Deserialize)]
//! pub struct BizActivity {
//!     pub id: Option<String>,
//!     pub name: Option<String>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let rb = RBatis::new();
//!     // init() also works, link() additionally checks that the url is reachable
//!     rb.link(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db").await?;
//!
//!     // exec() / query() return `rbs::Value`, exec_decode() decodes into `T`
//!     let rows: Vec<BizActivity> = rb
//!         .exec_decode("select * from biz_activity where id = ? limit ?", vec![to_value!("1"), to_value!(1)])
//!         .await?;
//!     println!("{:?}", rows);
//!
//!     let count: u64 = rb
//!         .exec_decode("select count(1) as count from biz_activity", vec![])
//!         .await?;
//!     println!("{}", count);
//!     Ok(())
//! }
//! ```
//!
//! # Transactions
//!
//! ```rust,no_run
//! use rbatis::{Error, RBatis};
//!
//! async fn do_tx(rb: &RBatis) -> Result<(), Error> {
//!     let tx = rb.acquire_begin().await?; // sends BEGIN
//!
//!     // ... run statements with `&tx` ...
//!
//!     tx.commit().await?;
//!     // tx.rollback().await?;
//!     Ok(())
//! }
//! ```

use crate::decode::decode;
use crate::intercept::{self, ResultType};
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

/// Anything that can execute SQL: [`crate::RBatis`], [`RBatisConnExecutor`],
/// [`RBatisTxExecutor`] and [`RBatisTxExecutorGuard`] all implement it.
///
/// The generated functions (`crud!`, `#[py_sql]`, `#[html_sql]`, `#[sql]`) take a
/// `&dyn Executor` as their first argument, so you can pass any of the types above
/// without changing the generated code.
///
/// Implement this trait only if you want to provide your own SQL front end; the
/// main entry points are [`Executor::exec`], [`Executor::query`] and the typed
/// helpers [`RBatisConnExecutor::exec_decode`] / [`RBatisTxExecutor::exec_decode`].
pub trait Executor: RBatisRef + Send + Sync {
    /// connection id or transaction id, also used as the `task_id` of interceptors
    fn id(&self) -> i64;

    /// the type name of this executor, used by interceptors to tell them apart
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// execute a statement (`insert`/`update`/`delete`/DDL) and return the affected row count
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    /// run a query and return the rows as [`rbs::Value`]
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
}

/// Access to the [`RBatis`] engine that owns this executor.
///
/// It is what makes `conn.rb_ref()` (for example in the [`table_sync`](crate::plugin::table_sync)
/// helpers) available on every executor.
pub trait RBatisRef: Any + Send + Sync {
    /// the engine this executor was created from
    fn rb_ref(&self) -> &RBatis;

    /// database driver name, for example `sqlite`, `mysql` or `postgres`
    fn driver_type(&self) -> crate::Result<&str> {
        self.rb_ref().driver_type()
    }
}

impl RBatisRef for RBatis {
    fn rb_ref(&self) -> &RBatis {
        self
    }
}

/// A checked out connection from the pool.
///
/// Holding this value keeps one connection for yourself, which is useful when you
/// want several statements to hit the same connection (temporary tables, `SET`s ...).
#[derive(Clone)]
pub struct RBatisConnExecutor {
    /// connection id, also used as the `task_id` of interceptors
    pub id: i64,
    /// the engine this connection was acquired from
    pub rb: RBatis,
    /// the underlying driver connection
    pub conn: Arc<Mutex<Box<dyn Connection>>>,
    /// interceptors of this engine
    pub intercepts: Arc<SyncVec<Arc<dyn crate::intercept::Intercept>>>,
}

impl RBatisConnExecutor {
    /// wrap an already opened driver connection
    pub fn new(id: i64, conn: Box<dyn Connection>, rb: RBatis) -> Self {
        Self {
            id,
            conn: Arc::new(Mutex::new(conn)),
            intercepts: rb.intercepts.clone(),
            rb: rb,
        }
    }

    /// take the connection out of this executor, if it is not shared anymore
    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        Arc::into_inner(self.conn).map(Mutex::into_inner)
    }
}

// `conn: Box<dyn Connection>` does not implement Debug, so Debug cannot be derived here.
impl Debug for RBatisConnExecutor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisConnExecutor")
            .field("id", &self.id)
            .field("rb", &self.rb)
            .finish_non_exhaustive()
    }
}

impl RBatisConnExecutor {
    /// execute a statement, see [`Executor::exec`]
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<ExecResult, Error> {
        let v = Executor::exec(self, sql, args).await?;
        Ok(v)
    }

    /// run a query, see [`Executor::query`]
    pub async fn query(&self, sql: &str, args: Vec<Value>) -> Result<Value, Error> {
        let v = Executor::query(self, sql, args).await?;
        Ok(v)
    }

    // Fast path for exec_decode - inlined to avoid trait method call overhead
    /// run a query and decode the rows into `T`
    pub async fn exec_decode<T>(&self, sql: &str, mut args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // Fast path: no interceptors - skip all overhead
        if self.intercepts.is_empty() {
            let result = self.conn.lock().await.exec_decode(sql, args).await;
            return result.and_then(|v| decode(v));
        }
        let mut sql = sql.to_string();
        let id = self.id;
        let mut before_result = Err(Error::from(""));
        if intercept::apply_before(
            &self.intercepts,
            id,
            self,
            &mut sql,
            &mut args,
            ResultType::Query(&mut before_result),
        )
        .await?
        {
            return before_result.and_then(|v| decode(v));
        }
        let mut conn = self.conn.lock().await;
        let mut args_after = args.clone();
        let mut result = conn.exec_decode(&sql, args).await;
        drop(conn);
        if intercept::apply_after(
            &self.intercepts,
            id,
            self,
            &mut sql,
            &mut args_after,
            ResultType::Query(&mut result),
        )
        .await?
        {
            return before_result.and_then(|v| decode(v));
        }
        result.and_then(|v| decode(v))
    }
}

impl Executor for RBatisConnExecutor {
    #[inline]
    fn id(&self) -> i64 {
        self.id
    }

    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let id = self.id;
            let mut before_result = Err(Error::from(""));
            if intercept::apply_before(
                &self.intercepts,
                id,
                self,
                &mut sql,
                &mut args,
                ResultType::Exec(&mut before_result),
            )
            .await?
            {
                return before_result;
            }
            let mut args_after = args.clone();
            let mut result = self.conn.lock().await.exec(&sql, args).await;
            if intercept::apply_after(
                &self.intercepts,
                id,
                self,
                &mut sql,
                &mut args_after,
                ResultType::Exec(&mut result),
            )
            .await?
            {
                return before_result;
            }
            result
        })
    }

    fn query(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let id = self.id;
            let mut before_result = Err(Error::from(""));
            if intercept::apply_before(
                &self.intercepts,
                id,
                self,
                &mut sql,
                &mut args,
                ResultType::Query(&mut before_result),
            )
            .await?
            {
                return before_result;
            }
            let mut conn = self.conn.lock().await;
            let mut args_after = args.clone();
            let mut result = conn.exec_decode(&sql, args).await;
            if intercept::apply_after(
                &self.intercepts,
                id,
                self,
                &mut sql,
                &mut args_after,
                ResultType::Query(&mut result),
            )
            .await?
            {
                return before_result;
            }
            result
        })
    }
}

impl RBatisRef for RBatisConnExecutor {
    fn rb_ref(&self) -> &RBatis {
        &self.rb
    }
}

impl RBatisConnExecutor {
    /// turn this connection into an open transaction ([`RBatisTxExecutor`])
    pub fn begin(self) -> BoxFuture<'static, Result<RBatisTxExecutor, Error>> {
        Box::pin(async move {
            let task_id = self.rb.task_id_generator.generate();
            let id = self.id;
            let rb = self.rb.clone();
            let conn = self.take_connection();
            let mut conn = conn.ok_or_else(|| Error::from("[rb] failed to take connection: connection Arc is still shared (this may happen if the executor was cloned)"))?;
            conn.begin().await?;
            let conn_executor = RBatisConnExecutor::new(id, conn, rb);
            Ok(RBatisTxExecutor::new(task_id, conn_executor))
        })
    }

    /// rollback the current transaction
    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { self.conn.lock().await.rollback().await })
    }

    /// commit the current transaction
    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async { self.conn.lock().await.commit().await })
    }
}

/// `RBatisTxExecutor` is a type that represents an executor for transactional operations in RBatis.
///
/// # Type Description
///
/// The `RBatisTxExecutor` is responsible for executing SQL statements within the context of a transaction.
/// It provides methods to execute queries, updates, and other SQL operations, ensuring that all operations
/// are part of the same transactional context.
#[derive(Clone, Debug)]
pub struct RBatisTxExecutor {
    /// transaction id, also used as the `task_id` of interceptors
    pub tx_id: i64,
    /// the connection this transaction runs on
    pub conn_executor: RBatisConnExecutor,
    /// please use tx.done()
    /// if tx call .commit() or .rollback() done = true.
    /// if tx not call .commit() or .rollback() done = false
    done: Arc<AtomicBool>,
}

impl RBatisTxExecutor {
    /// create a transaction executor from an open transaction id and connection
    pub fn new(tx_id: i64, conn_executor: RBatisConnExecutor) -> Self {
        RBatisTxExecutor {
            tx_id,
            conn_executor,
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
    pub async fn exec_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let v = Executor::query(self, sql, args).await?;
        decode(v)
    }

    /// send `BEGIN` on this connection
    pub fn begin(self) -> BoxFuture<'static, Result<Self, Error>> {
        Box::pin(async move {
            self.conn_executor.conn.lock().await.begin().await?;
            Ok(self)
        })
    }

    /// send `ROLLBACK` and mark this transaction as done
    pub fn rollback(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.conn_executor.conn.lock().await.rollback().await?;
            self.done.store(true, Ordering::Relaxed);
            Ok(())
        })
    }

    /// send `COMMIT` and mark this transaction as done
    pub fn commit(&self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async {
            self.conn_executor.conn.lock().await.commit().await?;
            self.done.store(true, Ordering::Relaxed);
            Ok(())
        })
    }

    /// tx is done?
    /// whether this transaction was committed or rolled back
    pub fn done(&self) -> bool {
        self.done.load(Ordering::Relaxed)
    }

    ///change is done
    /// change is done
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
    ///
    /// The returned [`RBatisTxExecutorGuard`] runs `callback` when it is dropped,
    /// which makes it impossible to forget a `rollback()` in an error path.
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

    /// auto commit or rollback when transaction is done
    ///
    /// If [`RBatisTxExecutor::done`] is `true` the transaction is committed when the
    /// guard is dropped, otherwise it is rolled back.
    pub fn auto_commit(&self) -> RBatisTxExecutorGuard {
        RBatisTxExecutorGuard {
            tx: self.clone(),
            callback: Arc::new(move |tx| {
                let future = async move {
                    if tx.done() {
                        let r = tx.commit().await;
                        if let Err(e) = r {
                            log::error!("transaction [{}] commit fail={}", tx.tx_id, e);
                        }
                    } else {
                        let r = tx.rollback().await;
                        if let Err(e) = r {
                            log::error!("transaction [{}] rollback fail={}", tx.tx_id, e);
                        }
                    }
                };
                rbdc::rt::spawn(future);
            }),
        }
    }
}

impl Executor for RBatisTxExecutor {
    fn id(&self) -> i64 {
        self.tx_id
    }

    fn exec(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let id = self.tx_id;
            let intercepts = &self.conn_executor.intercepts;
            let mut before_result = Err(Error::from(""));
            if intercept::apply_before(
                intercepts,
                id,
                self,
                &mut sql,
                &mut args,
                ResultType::Exec(&mut before_result),
            )
            .await?
            {
                return before_result;
            }
            let mut args_after = args.clone();
            let mut result = self.conn_executor.conn.lock().await.exec(&sql, args).await;
            if intercept::apply_after(
                intercepts,
                id,
                self,
                &mut sql,
                &mut args_after,
                ResultType::Exec(&mut result),
            )
            .await?
            {
                return before_result;
            }
            result
        })
    }

    fn query(&self, sql: &str, mut args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>> {
        let mut sql = sql.to_string();
        Box::pin(async move {
            let id = self.tx_id;
            let intercepts = &self.conn_executor.intercepts;
            let mut before_result = Err(Error::from(""));
            if intercept::apply_before(
                intercepts,
                id,
                self,
                &mut sql,
                &mut args,
                ResultType::Query(&mut before_result),
            )
            .await?
            {
                return before_result;
            }
            let mut conn = self.conn_executor.conn.lock().await;
            let mut args_after = args.clone();
            let mut result = conn.exec_decode(&sql, args).await;
            if intercept::apply_after(
                intercepts,
                id,
                self,
                &mut sql,
                &mut args_after,
                ResultType::Query(&mut result),
            )
            .await?
            {
                return before_result;
            }
            result
        })
    }
}

impl RBatisRef for RBatisTxExecutor {
    fn rb_ref(&self) -> &RBatis {
        &self.conn_executor.rb
    }
}

impl RBatisTxExecutor {
    /// take the connection out of this transaction executor
    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        self.conn_executor.take_connection()
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
    /// the wrapped transaction
    pub tx: RBatisTxExecutor,
    /// executed when the guard is dropped
    pub callback: Arc<dyn FnMut(RBatisTxExecutor) + Send + Sync>,
}

// `callback: Arc<dyn FnMut(..)>` does not implement Debug, so Debug cannot be derived here.
impl Debug for RBatisTxExecutorGuard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutorGuard")
            .field("tx", &self.tx)
            .finish_non_exhaustive()
    }
}

impl RBatisTxExecutorGuard {
    /// transaction id
    pub fn tx_id(&self) -> i64 {
        self.tx.tx_id
    }

    /// commit the transaction
    pub async fn commit(&self) -> crate::Result<()> {
        self.tx.commit().await?;
        Ok(())
    }

    /// rollback the transaction
    pub async fn rollback(&self) -> crate::Result<()> {
        self.tx.rollback().await?;
        Ok(())
    }

    /// take the connection out of the transaction
    pub fn take_connection(self) -> Option<Box<dyn Connection>> {
        self.tx.clone().take_connection()
    }

    /// run a query and decode the rows into `T`
    pub async fn exec_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.tx.exec_decode(sql, args).await
    }
}

impl Drop for RBatisTxExecutorGuard {
    fn drop(&mut self) {
        if let Some(callback) = Arc::get_mut(&mut self.callback) {
            callback(self.tx.clone());
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
    /// execute a statement (`insert`/`update`/`delete`/DDL) on a connection borrowed from the pool
    pub async fn exec(&self, sql: &str, args: Vec<Value>) -> Result<ExecResult, Error> {
        let conn = self.acquire().await?;
        conn.exec(sql, args).await
    }

    /// run a query on a connection borrowed from the pool and return the raw rows
    pub async fn query(&self, sql: &str, args: Vec<Value>) -> Result<Value, Error> {
        let conn = self.acquire().await?;
        let v = conn.query(sql, args).await?;
        Ok(v)
    }

    /// query and decode, superseded by [`RBatis::exec_decode`]
    #[deprecated(note = "use exec_decode instead")]
    pub async fn query_decode<T>(&self, sql: &str, args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let v = self.exec_decode(sql, args).await?;
        Ok(v)
    }

    /// run a query and decode the rows into `T`
    pub async fn exec_decode<T>(&self, sql: &str, mut args: Vec<Value>) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // Fast path: no interceptors - skip all overhead
        if self.intercepts.is_empty() {
            let pool = self
                .pool
                .get()
                .ok_or_else(|| Error::from("[rb] rbatis pool not inited!"))?;
            let mut conn = pool.get().await?;
            let result = conn.exec_decode(sql, args).await;
            return result.and_then(|v| decode(v));
        }

        let mut sql = sql.to_string();
        let mut before_result: Result<Value, Error> = Err(Error::from(""));
        if intercept::apply_before(
            &self.intercepts,
            0,
            self,
            &mut sql,
            &mut args,
            ResultType::Query(&mut before_result),
        )
        .await?
        {
            return before_result.and_then(|v| decode(v));
        }
        let pool = self
            .pool
            .get()
            .ok_or_else(|| Error::from("[rb] rbatis pool not inited!"))?;
        let mut conn = pool.get().await?;
        let mut args_after = args.clone();
        let mut result = conn.exec_decode(&sql, args).await;
        if intercept::apply_after(
            &self.intercepts,
            0,
            self,
            &mut sql,
            &mut args_after,
            ResultType::Query(&mut result),
        )
        .await?
        {
            return before_result.and_then(|v| decode(v));
        }
        result.and_then(|v| decode(v))
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
