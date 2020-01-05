use serde_json::Value;
use crate::core::rbatis::Rbatis;

pub struct Update {}

impl Update {
    pub fn update(&self, table: &str, arg: Value, engine: &Rbatis) -> Result<String, String> {
        unimplemented!()
        //TODO update by id
        //TODO update by ids
        //TODO update by map
        //TODO update by page
    }
}