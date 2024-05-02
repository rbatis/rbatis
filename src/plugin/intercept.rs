use crate::executor::Executor;
use crate::Error;
use async_trait::async_trait;
use rbdc::db::ExecResult;
use rbs::Value;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum ResultType<A, B> {
    /// Exec type
    Exec(/* #[pin] */ A),
    /// Query type
    Query(/* #[pin] */ B),
}

impl<A, B> ResultType<A, B> {
    pub fn type_name<'a, 'b>(&'a self) -> &'b str {
        match self {
            ResultType::Exec(_) => "exec",
            ResultType::Query(_) => "query",
        }
    }
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
///         result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
///     ) -> Result<Option<bool>, Error> {
///         Ok(Some(true))
///     }
/// }
/// ```
#[async_trait]
pub trait Intercept: Send + Sync + Debug {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    ///
    /// if return None will be return result
    /// if return Some(true) will be run next intercept
    /// if return Some(false) will be break
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        Ok(Some(true))
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
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        Ok(Some(true))
    }
}
