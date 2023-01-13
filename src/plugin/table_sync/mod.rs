pub mod sqlite_table_sync;
use crate::executor::RBatisConnExecutor;
use crate::Error;
use futures_core::future::BoxFuture;
use rbs::Value;
pub use sqlite_table_sync::*;

/// Note that it does not change the table structure.
/// If the table does not exist, it is created
/// If the table exists but a column is missing, increment the column of the missing section
pub trait TableSync {
    fn sync(
        &self,
        rb: RBatisConnExecutor,
        table: Value,
        name: &str,
    ) -> BoxFuture<Result<(), Error>>;
}
