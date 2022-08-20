#![warn(missing_debug_implementations, missing_docs)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(missing_debug_implementations, missing_docs)]

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
    pub fn into_ext(self, name: &'static str) -> Self {
        match self {
            Value::Ext(_, _) => self,
            _ => Value::Ext(name, Box::new(self)),
        }
    }
}

#[macro_export]
macro_rules! to_value {
    ($arg:expr) => {
        $crate::to_value($arg).unwrap_or_default()
    };
}
