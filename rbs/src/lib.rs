#![feature(ptr_internals)]
#![warn(missing_debug_implementations, missing_docs)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate serde;

#[allow(deprecated)]
pub mod value;

pub use crate::value::ext::to_value_ref;
pub use value::ext::{to_value,to_value_def};
pub use value::ext::{deserialize_from, from_value};
pub use value::{Value, ValueRef};


impl Value {
    pub fn into_ext(self, name: &'static str) -> Self {
        match self {
            Value::Ext(_, _) => {
                self
            }
            _ => {
                Value::Ext(name, Box::new(self))
            }
        }
    }
}

impl ValueRef<'_> {
    pub fn into_ext(self, name: &'static str) -> Self {
        match self {
            ValueRef::Ext(_, _) => {
                self
            }
            _ => {
                ValueRef::Ext(name, Box::new(self))
            }
        }
    }
}