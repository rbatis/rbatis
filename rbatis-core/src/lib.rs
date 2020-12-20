//! Core of rbatis::core, the rust SQL toolkit. Not intended to be used directly.

// #![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

#[macro_use]
extern crate lazy_static;

pub use error::{Error, Result};

pub mod runtime;

/// database
#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "mssql")]
mod mssql;

#[macro_use]
pub mod error;


pub mod results;


pub mod decode;

pub mod db;

pub mod convert;

pub mod sync;

pub mod value;