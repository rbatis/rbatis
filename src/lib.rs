pub extern crate dark_std;
pub extern crate rbatis_codegen;
extern crate rbatis_macro_driver;
pub extern crate rbdc;

pub use rbatis_macro_driver::{html_sql, py_sql, sql, snake_name};

pub mod plugin;

pub mod rbatis;
#[macro_use]
pub mod utils;
pub mod executor;
#[macro_use]
pub mod crud;
#[macro_use]
pub mod error;
pub mod decode;

pub mod sql;

pub use async_trait::async_trait;
pub use decode::*;
pub use error::*;
pub use plugin::*;
pub use rbatis::*;
pub use rbdc_pool_fast::FastPool as DefaultPool;
