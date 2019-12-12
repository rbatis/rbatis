use serde_json::Value;

pub struct Select {}

impl Select {
    pub fn eval(&self, table: &str, arg: Value) -> Result<String, String> {
        unimplemented!()
        //TODO select by id

        //TODO select by id vec

        //TODO select by map

        //TODO select by page
    }
}