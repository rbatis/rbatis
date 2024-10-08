use futures_core::future::BoxFuture;
use rbs::Value;


pub type Error = rbs::Error;

/// the Executor. this trait impl with structs = RBatis,RBatisConnExecutor,RBatisTxExecutor,RBatisTxExecutorGuard or any impl this struct
pub trait Executor: Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn exec(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>>;
    fn query(&self, sql: &str, args: Vec<Value>) -> BoxFuture<'_, Result<Value, Error>>;
    fn driver_type(&self) -> Result<&str, Error>;
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct ExecResult {
    pub rows_affected: u64,
    /// If some databases do not support last_insert_id, the default value is Null
    pub last_insert_id: Value,
}

impl std::fmt::Display for ExecResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct DisplayBox<'a> {
            inner: &'a Value,
        }
        impl<'a> std::fmt::Debug for DisplayBox<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.inner, f)
            }
        }
        f.debug_map()
            .key(&"rows_affected")
            .value(&self.rows_affected)
            .key(&"last_insert_id")
            .value(&DisplayBox {
                inner: &self.last_insert_id,
            })
            .finish()
    }
}

impl From<(u64, Value)> for ExecResult {
    fn from(value: (u64, Value)) -> Self {
        Self {
            rows_affected: value.0,
            last_insert_id: value.1,
        }
    }
}