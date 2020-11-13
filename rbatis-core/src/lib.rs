//! Core of rbatis::core, the rust SQL toolkit. Not intended to be used directly.

// When compiling with support for SQLite we must allow some unsafe code in order to
// interface with the inherently unsafe C module. This unsafe code is contained
// to the sqlite module.
#![cfg_attr(feature = "sqlite", deny(unsafe_code))]
// #![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

#[cfg(all(test, feature = "bench"))]
extern crate test;

// HACK: Allow a feature name the same name as a dependency
#[cfg(feature = "bigdecimal")]
extern crate bigdecimal_ as bigdecimal;

pub mod runtime;

mod mysql;
mod postgres;
mod sqlite;

#[macro_use]
pub mod error;

#[macro_use]
extern crate lazy_static;


pub mod decode;

pub use error::{Error, Result};

pub mod db_adapter;

pub mod convert;

pub mod sync;

pub mod db;

pub mod value;

pub mod mssql;