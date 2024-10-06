pub extern crate dark_std;
pub extern crate rbatis_codegen;
extern crate rbatis_macro_driver;
pub extern crate rbdc;
pub extern crate rbexec;

pub mod plugin;

pub mod rbatis;
#[macro_use]
pub mod utils;
pub mod executor;
// #[macro_use]
// pub mod crud;

#[macro_use]
pub mod error;
// pub mod decode;
pub use rbexec::decode as decode;

pub mod sql;
#[macro_use]
pub mod crud;


pub use async_trait::async_trait;
pub use decode::*;
pub use error::*;
pub use plugin::*;
pub use rbatis::*;
pub use rbdc_pool_fast::FastPool as DefaultPool;
pub use rbexec::{html_sql, py_sql, snake_name, sql};
