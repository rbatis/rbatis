#![feature(ptr_internals)]
#![warn(missing_debug_implementations, missing_docs)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate serde;
#[allow(deprecated)]
pub mod vbox;
pub use vbox::VBox;
pub mod value;
pub use crate::value::ext::to_value_ref;
pub use value::ext::to_value;
pub use value::ext::{deserialize_from, from_value};
pub use value::{Value, ValueRef};
