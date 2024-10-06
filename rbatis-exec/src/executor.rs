use futures_core::future::BoxFuture;
use rbdc::db::ExecResult;
use rbdc::Error;
use rbs::Value;

/// the Executor. this trait impl with structs = RBatis,RBatisConnExecutor,RBatisTxExecutor,RBatisTxExecutorGuard or any impl this struct
pub trait Executor: Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
    fn driver_type(&self) -> Result<&str, Error>;
}