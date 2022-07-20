use crate::CommonType;
use once_cell::sync::Lazy;
use rbs::Value;
use std::ops::Deref;

pub static JSONValue: Lazy<Value> = Lazy::new(|| Value::String("json".to_string()));

impl CommonType for Value {
    fn common_type(&self) -> &'static str {
        match self {
            Value::Null => {
                return "null";
            }
            Value::Bool(_) => {
                return "bool";
            }
            Value::I32(_) => {
                return "i32";
            }
            Value::I64(_) => {
                return "i64";
            }
            Value::U32(_) => {
                return "u32";
            }
            Value::U64(_) => {
                return "u64";
            }
            Value::F32(_) => {
                return "f32";
            }
            Value::F64(_) => {
                return "f64";
            }
            Value::String(s) => {
                return s.common_type();
            }
            Value::Binary(_) => {
                return "binary";
            }
            Value::Array(_) => {
                return "array";
            }
            Value::Map(_) => {
                return "map";
            }
            Value::Ext(_, _) => {
                return "ext";
            }
        }
    }
}
