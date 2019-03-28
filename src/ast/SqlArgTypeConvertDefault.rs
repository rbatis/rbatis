use crate::ast::SqlArgTypeConvert::SqlArgTypeConvert;
use serde_json::Value;

pub struct SqlArgTypeConvertDefault {
}

impl SqlArgTypeConvertDefault{
    pub fn new()->Self{
        return Self{};
    }
}

impl SqlArgTypeConvert for SqlArgTypeConvertDefault {
    fn convert(&self,arg: Value) -> String {
        match arg {
            Value::String(s) => return "'".to_owned()+s.as_str()+"'",
            Value::Number(n) => return n.to_string(),
            Value::Object(o) => panic!("not support object!"),
            _ => return String::from(""),
        }
    }
}