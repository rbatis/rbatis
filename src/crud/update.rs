use serde_json::Value;

pub struct Update {}

impl Update {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //update by id

        //update by id vec

        //update by map
    }
}