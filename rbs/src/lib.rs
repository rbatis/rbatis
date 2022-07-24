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
#[allow(deprecated)]

pub mod value;
pub use value::ext::to_value;
pub use value::ext::{deserialize_from, from_value};
pub use value::{Value, ValueRef};