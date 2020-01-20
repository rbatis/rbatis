use serde::de;

use crate::tx::propagation::Propagation;
use crate::tx::tx::TxImpl;

//pub trait Session<'a> {
//    fn id(&self) -> String;
//    fn query<T>(&mut self, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, String> where T: de::DeserializeOwned;
//    fn exec(&mut self, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64, String>;
//
//    fn rollback(&mut self) -> Result<u64, String>;
//    fn commit(&mut self) -> Result<u64, String>;
//    fn begin(&'a mut self, propagation_type: Propagation) -> Result<u64, String>;
//    fn close(&mut self);
//    fn last_propagation(&self) -> Option<Propagation>;
//}