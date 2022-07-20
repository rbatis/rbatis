use crate::Type;
use rbs::Value;

impl Type for Value {
    fn type_name(&self) -> &'static str {
        return match self {
            Value::Null => "null",
            Value::Bool(_) => "bool",
            Value::I32(_) => "i32",
            Value::I64(_) => "i64",
            Value::U32(_) => "u32",
            Value::U64(_) => "u64",
            Value::F32(_) => "f32",
            Value::F64(_) => "f64",
            Value::String(s) => s.type_name(),
            Value::Binary(_) => "binary",
            Value::Array(_) => "array",
            Value::Map(_) => "map",
            Value::Ext(_, _) => "ext",
        };
    }
}
