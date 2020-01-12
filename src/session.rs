use serde::de;
use serde_json::Value;
use crate::tx::propagation::Propagation;
use crate::tx::tx::Tx;


pub trait Session {
    fn id(&self) -> String;
    fn query<T>(&mut self,sql: &str, arg_array: &mut Vec<Value>) -> Result<T, String> where T: de::DeserializeOwned;
    fn exec(&mut self,sql: &str, arg_array: &mut Vec<Value>) -> Result<u64, String>;

    fn rollback(&mut self) -> Result<u64, String>;
    fn commit(&mut self) -> Result<u64, String>;
    fn begin(&mut self,propagation_type: Option<Propagation>) -> Result<Tx, String>;
    fn close(&mut self,);
    fn last_propagation(&self) -> Option<Propagation>;
}