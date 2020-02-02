use rdbc::Connection;
use serde::{Deserialize, Serialize};
use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::abstract_session::AbstractSession;
use crate::local_session::LocalSession;
use crate::tx::propagation::Propagation;
use crate::utils::driver_util;
use crate::utils::rdbc_util::to_rdbc_values;

///TX is transaction abstraction
/// Tx即事务抽象
pub trait Tx {
    fn id(&self) -> String;
    fn begin(id: &str, driver: &str, enable_log: bool, conn: &mut Box<dyn Connection>) -> Result<TxImpl, String>;
    fn query<T>(&mut self, sql: &str, arg_array: &[rdbc::Value], conn: &mut Box<dyn Connection>) -> Result<T, String> where T: de::DeserializeOwned;
    fn exec(&mut self, sql: &str, arg_array: &[rdbc::Value], conn: &mut Box<dyn Connection>) -> Result<u64, String>;
    fn rollback(&mut self, conn: &mut Box<dyn Connection>) -> Result<u64, String>;
    fn commit(&mut self, conn: &mut Box<dyn Connection>) -> Result<u64, String>;
    fn close(&mut self);
}

///TX is transaction abstraction
/// Tx即事务抽象
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxImpl {
    pub id: String,
    pub driver: String,
    //数据库连接，必须保
    pub is_close: bool,
    pub enable_log: bool,
}

impl TxImpl{
    fn do_begin(&mut self, conn: &mut Box<dyn Connection>) -> Result<u64, String> {
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        return conn.exec(true, "begin;", &[]);
    }
}

impl Tx for TxImpl {
    //开始一个事务
    fn begin(id: &str, driver: &str, enable_log: bool, conn: &mut Box<dyn Connection>) -> Result<TxImpl, String> {
        let mut v = id.to_string();
        if v.eq("") {
            v = Uuid::new_v4().to_string();
        }
        let mut s = Self {
            id: v,
            driver: driver.to_string(),
            is_close: false,
            enable_log: enable_log,
        };
        let data = s.do_begin(conn)?;
        return Ok(s);
    }

    fn id(&self) -> String {
        return self.id.clone();
    }

    fn query<T>(&mut self, sql: &str, arg_array: &[rdbc::Value], conn: &mut Box<dyn Connection>) -> Result<T, String> where T: de::DeserializeOwned {
        //let params = to_rdbc_values(arg_array);
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        return conn.query_prepare(self.enable_log, sql, &arg_array);
    }

    fn exec(&mut self, sql: &str, arg_array: &[rdbc::Value], conn: &mut Box<dyn Connection>) -> Result<u64, String> {
        //let params = to_rdbc_values(arg_array);
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        return conn.exec_prepare(self.enable_log, sql, &arg_array);
    }

    fn rollback(&mut self, conn: &mut Box<dyn Connection>) -> Result<u64, String> {
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        return conn.exec(true, "rollback;", &[]);
    }

    fn commit(&mut self, conn: &mut Box<dyn Connection>) -> Result<u64, String> {
        if self.is_close {
            return Err("[rbatis] conn is closed!".to_string());
        }
        return conn.exec(true, "commit;", &[]);
    }

    fn close(&mut self) {
        if self.is_close {
            return;
        }
        self.is_close = true;
    }
}

impl Drop for TxImpl {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
fn test_tx() {}