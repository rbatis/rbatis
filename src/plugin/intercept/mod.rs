//! SQL interceptors.
//!
//! An interceptor sees every statement that runs through an [`Executor`]:
//! [`Intercept::before`] is called before the statement is sent to the database and can
//! rewrite the SQL or the arguments (dynamic table names, read/write splitting, auditing,
//! `RETURNING` emulation, ...), [`Intercept::after`] is called with the result and can
//! replace it.
//!
//! Interceptors are installed on the engine with
//! [`RBatis::set_intercepts`](crate::RBatis::set_intercepts) and are looked up by
//! [`RBatis::get_intercept`](crate::RBatis::get_intercept).
//!
//! [`PageIntercept`](intercept_page::PageIntercept) (pagination) and
//! [`LogInterceptor`](intercept_log::LogInterceptor) (SQL logging) are installed by
//! default, more examples live in the
//! [example](https://github.com/rbatis/rbatis/tree/master/example) directory.

/// log every statement and its arguments through the `log` crate
pub mod intercept_log;
/// pagination interceptor, required by the `Page<T>` return type
pub mod intercept_page;

use crate::executor::Executor;
use crate::Error;
use async_trait::async_trait;
use dark_std::sync::SyncVec;
use rbdc::db::ExecResult;
use rbs::Value;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// Tells an interceptor whether the running statement is an `exec` (insert/update/
/// delete, result is [`ExecResult`]) or a `query` (result is an [`rbs::Value`]).
#[derive(Debug, Clone)]
pub enum ResultType<A, B> {
    /// exec type: an [`ExecResult`] is available
    Exec(/* #[pin] */ A),
    /// query type: an [`rbs::Value`] is available
    Query(/* #[pin] */ B),
}

impl<A, B> ResultType<A, B> {
    /// `"exec"` or `"query"`
    pub fn type_name<'b>(&self) -> &'b str {
        match self {
            ResultType::Exec(_) => "exec",
            ResultType::Query(_) => "query",
        }
    }
}

/// What the next interceptor should do.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// let the statement (or the next interceptor) run
    Next,
    /// stop here and return the result that the interceptor wrote into `result`
    Return,
}

/// SQL intercept
/// example:
///
/// ```rust
/// use rbatis::Error;
/// use rbatis::executor::Executor;
/// use rbatis::intercept::{Intercept, ResultType};
/// use rbdc::db::ExecResult;
/// use rbs::Value;
/// use rbatis::Action;
///
/// #[derive(Debug)]
/// pub struct ReturningIdPlugin{}
///
/// #[rbatis::async_trait]
/// impl Intercept for ReturningIdPlugin {
///     async fn before(
///         &self,
///         _task_id: i64,
///         rb: &dyn Executor,
///         sql: &mut String,
///         args: &mut Vec<Value>,
///         result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
///     ) -> Result<Action, Error> {
///         Ok(Action::Next)
///     }
/// }
/// ```
#[async_trait]
pub trait Intercept: Any + Send + Sync + Debug {
    /// name used by [`RBatis::get_intercept_dyn`](crate::RBatis::get_intercept_dyn),
    /// defaults to the type name of the interceptor
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        Ok(Action::Next)
    }

    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    /// return `Action::Next` to let the next interceptor run, return `Action::Return`
    /// to stop and use the result this interceptor wrote into `_result`
    async fn after(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
    ) -> Result<Action, Error> {
        Ok(Action::Next)
    }
}

/// Run before-interceptors. Returns `true` if an interceptor returned `Action::Return`.
/// The `result` parameter carries either `Exec` or `Query` variant to determine the result type.
pub async fn apply_before(
    intercepts: &SyncVec<Arc<dyn Intercept>>,
    id: i64,
    executor: &dyn Executor,
    sql: &mut String,
    args: &mut Vec<Value>,
    mut result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
) -> Result<bool, Error> {
    // copy the interceptors out of the container. SyncVec::iter() keeps this thread
    // registered as a reader until the iterator is dropped and a writer waits for every
    // reader, so iterating over the whole loop would keep a thread that adds or removes
    // an interceptor waiting for as long as this query runs - and would deadlock if an
    // interceptor added or removed one itself
    let interceptors: Vec<Arc<dyn Intercept>> = intercepts.iter().cloned().collect();
    for item in interceptors.iter() {
        let next = match &mut result {
            ResultType::Exec(r) => {
                item.before(id, executor, sql, args, ResultType::Exec(*r))
                    .await?
            }
            ResultType::Query(r) => {
                item.before(id, executor, sql, args, ResultType::Query(*r))
                    .await?
            }
        };
        if matches!(next, Action::Return) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Run after-interceptors. Returns `true` if an interceptor returned `Action::Return`.
/// The `result` parameter carries either `Exec` or `Query` variant to determine the result type.
pub async fn apply_after(
    intercepts: &SyncVec<Arc<dyn Intercept>>,
    id: i64,
    executor: &dyn Executor,
    sql: &mut String,
    args: &mut Vec<Value>,
    mut result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
) -> Result<bool, Error> {
    // the same reason as in apply_before
    let interceptors: Vec<Arc<dyn Intercept>> = intercepts.iter().cloned().collect();
    for item in interceptors.iter() {
        let next = match &mut result {
            ResultType::Exec(r) => {
                item.after(id, executor, sql, args, ResultType::Exec(*r))
                    .await?
            }
            ResultType::Query(r) => {
                item.after(id, executor, sql, args, ResultType::Query(*r))
                    .await?
            }
        };
        if matches!(next, Action::Return) {
            return Ok(true);
        }
    }
    Ok(false)
}
