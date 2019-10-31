use crate::ast::convert::SqlArgTypeConvert::SqlArgTypeConvert;
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
            Value::String(s) => {
                let mut ns=s;
                ns.insert_str(0,"'");
                ns=ns+"'";
                return ns;
            },
            Value::Number(n) => return n.to_string(),
            Value::Bool(b) => return b.to_string(),
            Value::Object(o) => panic!("[Rbatis] not support convert Object/Map<String,Value>!"),
            Value::Array(arr) => panic!("[Rbatis] not support convert Vec<Value>!"),
            _ => return String::from(""),
        }
    }
}