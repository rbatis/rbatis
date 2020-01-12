use rdbc::Connection;
use crate::session::Session;
use crate::tx::propagation::Propagation;
use serde_json::Value;
use serde::de;

pub struct Tx{
    pub coon:Box<dyn Connection>,
    pub is_start:bool,
    pub is_close:bool,
}

impl Tx{

}

impl Session for Tx{
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

    fn begin(&mut self, propagation_type: Option<Propagation>) -> Result<u64, String> {
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
fn test_tx(){
    
}