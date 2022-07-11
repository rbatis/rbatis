use rbdc::db::{Connection, Statement};
use rbdc::Error;

pub struct MysqlConn {}

impl Connection for MysqlConn {
    fn create(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>, Error> {
        todo!()
    }

    fn prepare(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>, Error> {
        todo!()
    }
}
