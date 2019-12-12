use serde_json::Value;

pub struct Delete {}

impl Delete {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //delete by id

        //delete by id vec

        //delete by map
    }
}