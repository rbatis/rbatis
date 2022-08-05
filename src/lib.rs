#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rbatis_macro_driver;
#[macro_use]
pub extern crate rbatis_sql;

pub use rbatis_core as core;
pub use rbatis_core::*;
pub use rbatis_sql::ops::*;
pub use rbatis_sql::{expr, push_index, rb_html, rb_py, sql_index};

pub use rbatis_macro_driver::{html_sql, py_sql, sql};

pub use crate::core::{convert::StmtConvert, error::Error, error::Result};
pub mod plugin;

pub use plugin::*;

pub mod rbatis;
#[macro_use]
pub mod utils;
pub mod executor;
pub mod sql;
#[macro_use]
pub mod crud;
pub use crud::*;
