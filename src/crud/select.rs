use serde_json::Value;

pub struct Select {}

impl Select {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //TODO select by id

        //TODO select by id vec

        //TODO select by map

        //TODO select by page
    }
}