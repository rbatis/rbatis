use serde_json::Value;
use crate::core::rbatis::Rbatis;

pub trait Templete {
    fn eval(&self, table: &str, arg: Value, engine: &Rbatis) -> Result<String, String>;
}