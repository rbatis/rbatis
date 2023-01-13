pub extern crate dark_std;
pub extern crate rbatis_codegen;
extern crate rbatis_macro_driver;
pub extern crate rbdc;

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

pub use crate::rbatis::Rbatis;
pub use crate::rbatis::RbatisOption;
pub use crud::*;
pub use decode::decode;
pub use error::Error;
pub use error::Result;
