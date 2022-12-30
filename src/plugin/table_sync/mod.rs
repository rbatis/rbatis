pub mod sqlite_table_sync;
pub use sqlite_table_sync::*;

use crate::executor::RBatisConnExecutor;
use crate::utils::string_util::to_snake_name;
use crate::Error;
use futures_core::future::BoxFuture;
use rbdc::db::Connection;
use rbs::{to_value, Value};
use serde::Serialize;
use std::any::Any;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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