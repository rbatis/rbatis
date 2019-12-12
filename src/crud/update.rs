use serde_json::Value;

pub struct Update {}

impl Update {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //TODO update by id

        //TODO update by id vec

        //TODO update by map
    }
}