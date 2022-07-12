use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use rbdc::db::{ResultSet, Statement};
use rbdc::Error;
use rbdc::ext::ustr::UStr;

pub struct MysqlStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) metadata: MySqlStatementMetadata,
}

impl <'q>Statement for MysqlStatement<'q> {
    fn fetch(&mut self, params: &[rbs::value::Value]) -> Result<Box<dyn ResultSet + '_>, Error> {
        todo!()
    }

    fn exec(&mut self, params: &[rbs::value::Value]) -> Result<u64, Error> {
        todo!()
    }
}


#[derive(Debug, Default, Clone)]
pub(crate) struct MySqlStatementMetadata {
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
