use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use futures_core::future::BoxFuture;
use rbdc::db::Row;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MySqlStatement {
    pub sql: String,
    pub metadata: MySqlStatementMetadata,
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

impl From<Vec<rbs::Value>> for MySqlArguments {
    fn from(arg: Vec<rbs::Value>) -> Self {
        for x in arg {
            match x {
                Value::Nil => {}
                Value::Bool(_) => {}
                Value::I32(_) => {}
                Value::I64(_) => {}
                Value::U32(_) => {}
                Value::U64(_) => {}
                Value::F32(_) => {}
                Value::F64(_) => {}
                Value::String(_) => {}
                Value::Binary(_) => {}
                Value::Array(_) => {}
                Value::Map(ref m) => {
                    let v = &x["type"];
                }
                Value::Ext(_, _) => {}
            }
        }
        todo!()
    }
}
