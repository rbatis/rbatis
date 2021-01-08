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

pub use rbatis_macro_driver::{CRUDEnable,crud_enable,sql,py_sql};


pub use rbatis_core as core;

pub mod utils;
pub mod interpreter;
pub mod plugin;
pub mod rbatis;
pub mod sql;
pub mod crud;
pub mod wrapper;
pub mod tx;

