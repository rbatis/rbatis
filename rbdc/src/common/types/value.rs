use crate::CommonType;
use once_cell::sync::Lazy;
use rbs::Value;

pub static JSONValue: Lazy<Value> = Lazy::new(|| Value::String("json".to_string()));

impl CommonType for Value {
    fn common_type(&self) -> &'static str {
        match self {
            Value::Null => {}
            Value::Bool(_) => {}
            Value::I32(_) => {}
            Value::I64(_) => {}
            Value::U32(_) => {}
            Value::U64(_) => {}
            Value::F32(_) => {}
            Value::F64(_) => {}
            Value::String(_) => {}
            Value::Binary(_) => {}
            Value::Array(arr) => {
                if arr.len() == 1 && arr[0].eq(&JSONValue) {
                    return "json";
                }
            }
            Value::Map(_) => {}
            Value::Ext(_, _) => {}
        }
        todo!()
    }
}
