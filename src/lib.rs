#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;
extern crate once_cell;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate rbatis_macro_driver;

pub use crate::core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};
pub use rbatis_core as core;
pub use rbatis_macro_driver::{crud_enable, py_sql, sql, CRUDEnable};

pub mod crud;
pub mod plugin;
pub mod rbatis;
pub mod sql;
pub mod tx;
pub mod utils;
pub mod wrapper;
