use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use futures_core::future::BoxFuture;
use rbdc::db::{ResultSet, Statement};
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MySqlStatement {
    pub sql: String,
    pub metadata: MySqlStatementMetadata,
}

impl Statement for MySqlStatement {
    fn fetch(&mut self, params: &[Value]) -> BoxFuture<Result<Box<dyn ResultSet>, Error>> {
        Box::pin(async move { todo!() })
    }

    fn exec(&mut self, params: &[Value]) -> BoxFuture<Result<u64, Error>> {
        Box::pin(async move { todo!() })
    }
}

#[derive(Debug, Default, Clone)]
pub struct MySqlStatementMetadata {
    pub(crate) columns: Arc<Vec<MySqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
    pub(crate) parameters: usize,
}

/// Implementation of [`Arguments`] for MySQL.
#[derive(Debug, Default, Clone)]
pub struct MySqlArguments {
    pub values: Vec<u8>,
    pub types: Vec<MySqlTypeInfo>,
    pub null_bitmap: Vec<u8>,
}
