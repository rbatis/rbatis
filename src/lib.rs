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
extern crate rbatis_macro_driver;
#[macro_use]
pub extern crate rbatis_sql;
#[macro_use]
extern crate serde_json;

pub use rbatis_core as core;
pub use rbatis_sql::{expr, push_index, rb_html, rb_py, sql_index};
pub use rbatis_sql::ops::*;

pub use rbatis_macro_driver::{crud_table, CRUDTable, html_sql, py_sql, sql};

pub use crate::core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};


pub mod crud;
pub mod plugin;
pub mod rbatis;
pub mod sql;
#[macro_use]
pub mod utils;
pub mod wrapper;
pub mod executor;