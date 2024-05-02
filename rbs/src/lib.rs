#[macro_use]
extern crate serde;
extern crate core;

pub mod index;
pub mod value;

pub use crate::value::ext::Error;
pub use value::ext::{from_value, from_value_ref};
pub use value::ext::{to_value, to_value_def};
pub use value::Value;

impl Value {
    pub fn into_ext(self, name: &'static str) -> Self {
        match self {
            Value::Ext(_, _) => self,
            _ => Value::Ext(name, Box::new(self)),
        }
    }

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
            Value::Ext(_, v) => v.is_empty(),
        }
    }

    /// return array/map/string's length
    pub fn len(&self) -> usize {
        match self {
            Value::Null => 0,
            Value::Bool(_) => 0,
            Value::I32(_) => 0,
            Value::I64(_) => 0,
            Value::U32(_) => 0,
            Value::U64(_) => 0,
            Value::F32(_) => 0,
            Value::F64(_) => 0,
            Value::String(v) => v.len(),
            Value::Binary(v) => v.len(),
            Value::Array(v) => v.len(),
            Value::Map(v) => v.len(),
            Value::Ext(_, v) => v.len(),
        }
    }
}

/// to_value macro
///
/// to_value! map
///```rust
/// let v=  rbs::to_value! {"1":"1",};
///```
/// to_value! expr
///```rust
/// let arg="1";
/// let v =  rbs::to_value!(arg);
///```
#[macro_export]
macro_rules! to_value {
    ($($k:tt: $v:expr),* $(,)?) => {
       $crate::Value::Map($crate::value_map!($($k:$v ,)*))
    };
    ($arg:expr) => {
        $crate::to_value($arg).unwrap_or_default()
    };
}

/// is debug mode
pub fn is_debug_mode() -> bool {
    if cfg!(debug_assertions) {
        #[cfg(feature = "debug_mode")]
        {
            true
        }
        #[cfg(not(feature = "debug_mode"))]
        {
            false
        }
    } else {
        false
    }
}
