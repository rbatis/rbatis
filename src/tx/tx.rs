use rdbc::Connection;
use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::local_session::LocalSession;
use crate::session::Session;
use crate::tx::propagation::Propagation;
use crate::utils::driver_util;
use crate::query_impl::Queryable;

pub struct Tx {
    pub id: String,
    pub driver: String,
    pub conn: Box<dyn Connection>,
    pub is_close: bool,
}

impl Tx {
    pub fn new(driver: &str) -> Result<Self, String> {
        let r = driver_util::get_conn_by_link(driver)?;
        return Ok(Self {
            id: Uuid::new_v4().to_string(),
            driver: driver.to_string(),
            conn: r,
            is_close: false,
        });
    }
}

impl Session for Tx {
    fn id(&self) -> String {
        unimplemented!()
    }

    fn query<T>(&mut self, sql: &str, arg_array: &mut Vec<serde_json::Value>) -> Result<T, String> where T: de::DeserializeOwned {
       //return self.conn.query(sql,arg_array);
        unimplemented!()
    }

    fn exec(&mut self, sql: &str, arg_array: &mut Vec<serde_json::Value>) -> Result<u64, String> {
       // return self.conn.exec(sql,arg_array);
        unimplemented!()
    }

    fn rollback(&mut self) -> Result<u64, String> {
        unimplemented!()
    }

    fn commit(&mut self) -> Result<u64, String> {
        unimplemented!()
    }

    fn begin(&mut self, propagation_type: Option<Propagation>) -> Result<Tx, String> {
        unimplemented!()
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn last_propagation(&self) -> Option<Propagation> {
        unimplemented!()
    }
}


#[test]
fn test_tx() {}