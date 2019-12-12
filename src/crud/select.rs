use serde_json::Value;

pub struct Select {}

impl Select {
    pub fn eval<T>(&self, table: &str, arg: Value) -> Result<T, String> {
        unimplemented!()
        //select by id

        //select by id vec

        //select by map

        //select by page
    }
}