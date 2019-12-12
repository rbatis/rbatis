use serde_json::Value;

pub struct Delete {}

impl Delete {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //TODO delete by id

        //TODO delete by id vec

        //TODO delete by map
    }
}