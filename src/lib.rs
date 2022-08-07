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

pub use rbatis_sql::ops::*;
pub use rbatis_sql::{expr, push_index, rb_html, rb_py, sql_index};

pub use rbatis_macro_driver::{html_sql, py_sql, sql};

pub mod plugin;

pub use plugin::*;

pub mod rbatis;
#[macro_use]
pub mod utils;
pub mod executor;
pub mod sql;
#[macro_use]
pub mod crud;
#[macro_use]
pub mod error;
pub mod decode;

pub use crud::*;
pub use error::Error;
pub use error::Result;
pub use crate::rbatis::Rbatis;
pub use crate::rbatis::RbatisOption;
