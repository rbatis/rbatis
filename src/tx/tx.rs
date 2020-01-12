use rdbc::Connection;
use serde::de;
use serde_json::Value;

use crate::local_session::LocalSession;
use crate::session::Session;
use crate::tx::propagation::Propagation;

pub struct Tx {
    pub session: Option<LocalSession>,
    pub is_close: bool,
}

impl Tx {
    pub fn new()->Self{
        return Self{
            session: None ,
            is_close: false
        }
    }
}

impl Session for Tx {
    fn id(&self) -> String {
        unimplemented!()
    }

    fn query<T>(&mut self, sql: &str, arg_array: &mut Vec<Value>) -> Result<T, String> where T: de::DeserializeOwned {
        unimplemented!()
    }

    fn exec(&mut self, sql: &str, arg_array: &mut Vec<Value>) -> Result<u64, String> {
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

    fn propagation(&self) -> Option<Propagation> {
        unimplemented!()
    }
}


#[test]
fn test_tx() {}