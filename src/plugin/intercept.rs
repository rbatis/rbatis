use crate::executor::Executor;
use crate::Error;
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

/// sql intercept
pub trait Intercept: Send + Sync + Debug {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    /// if set result = Some,will be return result
    fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Option<ExecResult>, &mut Option<Vec<Value>>>,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// task_id maybe is conn_id or tx_id,
    /// is_prepared_sql = !args.is_empty(),
    fn after(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[deprecated(note = "please use Intercept replace this")]
pub type SqlIntercept = dyn Intercept;
