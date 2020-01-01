
use crate::ast::convert::sql_arg_type_convert::SqlArgTypeConvert;
use serde_json::{json, Value};

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
            Value::Object(o) => panic!("[rbatis] not support convert Object/Map<String,Value>!"),
            Value::Array(arr) => panic!("[rbatis] not support convert Vec<Value>!"),
            _ => return String::from(""),
        }
    }
}


#[test]
fn test_convert(){
    let convert=SqlArgTypeConvertDefault{};
    let mut result;
    result =  convert.convert(json!(1));
    println!("number(i64)=>{}",result);
    result =  convert.convert(json!(1.2));
    println!("number(f64)=>{}",result);
    result =  convert.convert(json!("abc"));
    println!("string=>{}",result);
    result =  convert.convert(json!(null));
    println!("null=>{}",result);
}
