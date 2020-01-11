use serde::de;
use serde_json::Value;
use uuid::Uuid;

use crate::core::session::Session;
use crate::tx::propagation::Propagation;
use crate::tx::save_point_stack::SavePointStack;
use crate::tx::tx_stack::TxStack;

pub struct LocalSession {
    pub session_id: String,
    pub driver: String,
    pub tx_stack: TxStack,
    pub save_point_stack: SavePointStack,
    pub is_closed: bool,
    pub new_local_session: Option<Box<LocalSession>>,
}

impl LocalSession {
    pub fn new(driver: &str) -> Self {
        return Self {
            session_id: Uuid::new_v4().to_string(),
            driver: driver.to_string(),
            tx_stack: TxStack::new(),
            save_point_stack: SavePointStack::new(),
            is_closed: false,
            new_local_session: None,
        };
    }
}

impl Session for LocalSession {
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

    fn begin(&mut self, propagation_type:  Option<Propagation>) -> Result<u64, String> {
        if propagation_type.is_some(){
            match propagation_type.as_ref().unwrap() {
                Propagation::REQUIRED=>{
                    if self.tx_stack.len()>0{

                    }
                }
                Propagation::NOT_SUPPORTED => {
                    if self.tx_stack.len() > 0 {
                        //TODO stop old tx
                    }
                    self.new_local_session = Some(Box::new(LocalSession::new(self.driver.as_str())));
                }
                _ => {}
            }
        }
        return Ok(0);
    }

    fn close(&mut self) {
        unimplemented!()
    }

    fn propagation(&self) -> Option<Propagation> {
        unimplemented!()
    }
}