use rdbc::Connection;
use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::local_session::LocalSession;
use crate::queryable::Queryable;
use crate::session::Session;
use crate::tx::propagation::Propagation;
use crate::utils::driver_util;
use crate::utils::rdbc_util::to_rdbc_values;

pub struct Tx<'a> {
    pub id: String,
    pub driver: String,
    pub conn: Option<&'a mut Box<dyn Connection>>,
    pub is_close: bool,
    pub enable_log: bool,
}

impl<'a> Tx<'a> {
    //开始一个事务
    pub fn begin(id: &str, driver: &str, enable_log: bool, conn: Option<&'a mut Box<dyn Connection>>) -> Result<Tx<'a>,String> {
        let mut v = id.to_string();
        if v.eq("") {
            v = Uuid::new_v4().to_string();
        }
        let mut s= Self {
            id: v,
            driver: driver.to_string(),
            conn: conn,
            is_close: false,
            enable_log: enable_log,
        };
        let data=s.conn.as_mut().unwrap().exec(enable_log,"begin;",&[])?;
        return Ok(s);
    }

    pub fn id(&self) -> String {
        return self.id.clone();
    }

    pub fn query<T>(&mut self, sql: &str, arg_array: &mut Vec<serde_json::Value>) -> Result<T, String> where T: de::DeserializeOwned {
        let params = to_rdbc_values(arg_array);
        return self.conn.as_mut().unwrap().query(self.enable_log, sql, &params);
    }

    pub fn exec(&mut self, sql: &str, arg_array: &mut Vec<serde_json::Value>) -> Result<u64, String> {
        let params = to_rdbc_values(arg_array);
        return self.conn.as_mut().unwrap().exec(self.enable_log, sql, &params);
    }

    pub fn rollback(&mut self) -> Result<u64, String> {
        return self.conn.as_mut().unwrap().exec(true, "rollback;", &[]);
    }

    pub fn commit(&mut self) -> Result<u64, String> {
        return self.conn.as_mut().unwrap().exec(true, "commit;", &[]);
    }
}

#[test]
fn test_tx() {

}