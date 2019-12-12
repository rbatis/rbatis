use serde_json::Value;

pub trait Templete {
    fn eval(&self, table: &str, arg: Value) -> Result<String, String>;
}