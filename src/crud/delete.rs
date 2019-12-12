use serde_json::Value;

pub struct Delete {}

impl Delete {
    pub fn eval(&self, table: &str, arg: Value) -> Result<String, String> {
        unimplemented!()
        //delete by id

        //delete by id vec

        //delete by map
    }
}