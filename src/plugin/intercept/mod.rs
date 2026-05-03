pub mod intercept_log;
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

#[derive(Debug, Clone)]
pub enum ResultType<A, B> {
    /// Exec type
    Exec(/* #[pin] */ A),
    /// Query type
    Query(/* #[pin] */ B),
}

impl<A, B> ResultType<A, B> {
    pub fn type_name<'b>(&self) -> &'b str {
        match self {
            ResultType::Exec(_) => "exec",
            ResultType::Query(_) => "query",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// run next
    Next,
    /// return result
    Return,
}

/// sql intercept
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
    /// if return Ok(false) will be return data. return Ok(true) will run next
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
    for item in intercepts.iter() {
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
    for item in intercepts.iter() {
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
