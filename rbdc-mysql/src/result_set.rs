use rbdc::db::{MetaData, ResultSet};
use rbdc::Error;

pub struct MysqlResultSet {}

impl ResultSet for MysqlResultSet {
    fn meta_data(&self) -> Result<Box<dyn MetaData>, Error> {
        todo!()
    }

    fn next(&mut self) -> bool {
        todo!()
    }

    fn get(&self, i: u64) -> Result<rbs::value::Value, Error> {
        todo!()
    }
}
