use serde_json::Value;
use crate::core::rbatis::Rbatis;

pub struct Delete {}

impl Delete {
    pub fn eval(&self, table: &str, arg: Value, engine: &Rbatis) -> Result<String, String> {
        unimplemented!()
        //TODO delete by id

        //TODO delete by id vec

        //TODO delete by map
    }
}