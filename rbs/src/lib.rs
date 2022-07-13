//! This crate connects Rust MessagePack library with [`serde`][serde] providing an ability to
//! easily serialize and deserialize both Rust built-in types, the standard library and custom data
//! structures.
//!
//! ## Motivating example
//!
//! ```
//! let buf = rmp_serde::to_vec(&(42, "the Answer")).unwrap();
//!
//! assert_eq!(
//!     vec![0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72],
//!     buf
//! );
//!
//! assert_eq!((42, "the Answer"), rmp_serde::from_read_ref(&buf).unwrap());
//! ```
//!
//! # Type-based Serialization and Deserialization
//!
//! Serde provides a mechanism for low boilerplate serialization & deserialization of values to and
//! from MessagePack via the serialization API.
//!
//! To be able to serialize a piece of data, it must implement the `serde::Serialize` trait. To be
//! able to deserialize a piece of data, it must implement the `serde::Deserialize` trait. Serde
//! provides an annotation to automatically generate the code for these
//! traits: `#[derive(Serialize, Deserialize)]`.
//!
//! # Examples
//!
//! ```
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate rbs;
//!
//! use std::collections::HashMap;
//! use serde::{Deserialize, Serialize};
//! use rbs::{Deserializer, Serializer};
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Human {
//!     age: u32,
//!     name: String,
//! }
//!
//! fn main() {
//!     let mut buf = Vec::new();
//!     let val = Human {
//!         age: 42,
//!         name: "John".into(),
//!     };
//!
//!     val.serialize(&mut Serializer::new(&mut buf)).unwrap();
//! }
//! ```
//!
//! [serde]: https://serde.rs/

#![warn(missing_debug_implementations, missing_docs)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate serde;

use std::fmt::{self, Display, Formatter};
use std::mem;
use std::str::{self, Utf8Error};

use serde::de;
use serde::{Deserialize, Serialize};

#[allow(deprecated)]
pub use crate::decode::from_read_ref;
pub use crate::decode::{from_read, Deserializer};
pub use crate::encode::{to_vec, to_vec_named, Serializer};

pub use crate::decode::from_slice;

pub mod config;
pub mod decode;
pub mod encode;
pub mod value;

pub use value::decode::{
    read_value, read_value_ref, read_value_ref_with_max_depth, read_value_with_max_depth,
};
pub use value::encode::to_value_ref;
pub use value::ext::to_value;
pub use value::ext::{deserialize_from, from_value};
pub use value::{Value, ValueRef};

/// Name of Serde newtype struct to Represent Msgpack's Ext
/// Msgpack Ext: Ext(tag, binary)
/// Serde data model: _ExtStruct((tag, binary))
/// Example Serde impl for custom type:
///
/// ```ignore
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// #[serde(rename = "_ExtStruct")]
/// struct ExtStruct((i8, serde_bytes::ByteBuf));
///
/// test_round(ExtStruct((2, serde_bytes::ByteBuf::from(vec![5]))),
///            Value::Ext(2, vec![5]));
/// ```
pub const MSGPACK_EXT_STRUCT_NAME: &str = "_ExtStruct";
