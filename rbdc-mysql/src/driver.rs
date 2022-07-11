use rbdc::db::{Connection, Driver};
use rbdc::Error;

pub struct MysqlDriver {}

impl Driver for MysqlDriver {
    fn connect(&self, url: &str) -> Result<Box<dyn Connection>, Error> {
        todo!()
    }
}
