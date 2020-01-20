use rdbc::Connection;
use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::local_session::LocalSession;
use crate::queryable::Queryable;
use crate::tx::propagation::Propagation;
use crate::utils::driver_util;
use crate::utils::rdbc_util::to_rdbc_values;

pub struct Tx {
    pub id: String,
    pub driver: String,
    pub conn: *mut Box<dyn Connection>,
    //数据库连接，必须保
    pub is_close: bool,
    pub enable_log: bool,
}

impl Tx {
    //开始一个事务
    pub fn begin(id: &str, driver: &str, enable_log: bool, conn: *mut Box<dyn Connection>) -> Result<Tx, String> {
        let mut v = id.to_string();
        if v.eq("") {
            v = Uuid::new_v4().to_string();
        }
        let mut s = Self {
            id: v,
            driver: driver.to_string(),
            conn: conn,
            is_close: false,
            enable_log: enable_log,
        };
        let data = s.exec("begin;", &[])?;
        return Ok(s);
    }

    pub fn id(&self) -> String {
        return self.id.clone();
    }

    pub fn query<T>(&mut self, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, String> where T: de::DeserializeOwned {
        //let params = to_rdbc_values(arg_array);
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        unsafe {
            return self.conn.as_mut().unwrap().query(self.enable_log, sql, &arg_array);
        }
    }

    pub fn exec(&mut self, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64, String> {
        //let params = to_rdbc_values(arg_array);
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        unsafe {
            return self.conn.as_mut().unwrap().exec(self.enable_log, sql, &arg_array);
        }
    }

    pub fn rollback(&mut self) -> Result<u64, String> {
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        unsafe {
            return self.conn.as_mut().unwrap().exec(true, "rollback;", &[]);
        }
    }

    pub fn commit(&mut self) -> Result<u64, String> {
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        unsafe {
            return self.conn.as_mut().unwrap().exec(true, "commit;", &[]);
        }
    }
}

#[test]
fn test_tx() {}