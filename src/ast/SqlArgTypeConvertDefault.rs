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
            Value::Null => return String::from("null"),
            Value::String(s) => return "'".to_owned()+s.as_str()+"'",
            Value::Number(n) => return n.to_string(),
            Value::Bool(b) => return b.to_string(),
            Value::Object(o) => panic!("[RustMybatis] not support convert object!"),
            Value::Array(arr) => panic!("[RustMybatis] not support convert array!"),
            _ => return String::from(""),
        }
    }
}