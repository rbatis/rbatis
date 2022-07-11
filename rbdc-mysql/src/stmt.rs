use crate::result_set::MySqlTypeInfo;
use rbdc::db::{ResultSet, Statement};
use rbdc::Error;

pub struct MysqlStatement {}

impl Statement for MysqlStatement {
    fn fetch(&mut self, params: &[rbs::value::Value]) -> Result<Box<dyn ResultSet + '_>, Error> {
        todo!()
    }

    fn exec(&mut self, params: &[rbs::value::Value]) -> Result<u64, Error> {
        todo!()
    }
}

/// Implementation of [`Arguments`] for MySQL.
#[derive(Debug, Default, Clone)]
pub struct MySqlArguments {
    pub values: Vec<u8>,
    pub types: Vec<MySqlTypeInfo>,
    pub null_bitmap: Vec<u8>,
}
