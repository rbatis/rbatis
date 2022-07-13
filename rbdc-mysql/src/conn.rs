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

#[cfg(test)]
mod test {
    use rbs::Value;

    #[test]
    fn test_mysql() {
        let v = Value::Array(vec![Value::F32(1f32)]);
        println!("{}", v);
    }
}
