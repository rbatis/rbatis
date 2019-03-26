use crate::ast::SqlArgTypeConvert::SqlArgTypeConvert;
use serde_json::Value;

pub struct SqlArgTypeConvertDefault {}

impl SqlArgTypeConvert for SqlArgTypeConvertDefault {
    fn convert(arg: Value) -> String {
        match arg {
            Value::Null => return String::from("''"),
            Value::String(s) => return s,
            Value::Number(n) => return n.to_string(),
            Value::Object(o) => panic!("not support object!"),
            _ => return String::from(""),
        }
    }
}