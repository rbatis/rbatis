use serde_json::Value;

pub struct Delete{

}

impl Delete{
    pub  fn eval(&self, table: &str, arg: Value) -> Result<String,String>{
        unimplemented!()
    }
}