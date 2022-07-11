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
