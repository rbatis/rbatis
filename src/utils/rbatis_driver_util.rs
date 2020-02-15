use serde_json::Value;

use crate::convert::sql_value_convert::{SkipType, SqlValueConvert};

pub fn to_driver_values(arg_array: &mut Vec<Value>) -> Vec<rbatis_drivers::Value> {
    let mut params = vec![];
    for x in arg_array {
        let item = x.to_sql_value_skip(false,SkipType::None);
        params.push(rbatis_drivers::Value::String(item));
    }
    return params;
}

pub trait FormatString{
    fn format_string(&self) -> String;
}

impl FormatString for &[rbatis_drivers::Value]{
    fn format_string(&self) -> String {
        let mut s = String::new();
        for x in *self {
            s = s + x.to_string().as_str() + ",";
        }
        if s.len() > 0 {
            s.pop();
            s = "[".to_string() + s.as_str();
            s = s + "]";
            return s;
        } else {
            return "[]".to_string();
        }
    }
}