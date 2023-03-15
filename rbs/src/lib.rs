#[macro_use]
extern crate serde;
extern crate core;

pub mod index;
#[allow(deprecated)]
pub mod value;

pub use crate::value::ext::Error;
pub use value::ext::{deserialize_from, from_value};
pub use value::ext::{to_value, to_value_def};
pub use value::Value;

impl Value {
    pub fn is_empty(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Bool(_) => false,
            Value::I32(_) => false,
            Value::I64(_) => false,
            Value::U32(_) => false,
            Value::U64(_) => false,
            Value::F32(_) => false,
            Value::F64(_) => false,
            Value::String(v) => v.is_empty(),
            Value::Binary(v) => v.is_empty(),
            Value::Array(v) => v.is_empty(),
            Value::Map(v) => v.is_empty(),
        }
    }
}

#[macro_export]
macro_rules! to_value {
    ($arg:expr) => {
        $crate::to_value($arg).unwrap_or_default()
    };
}
