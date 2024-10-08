pub extern crate dark_std;
pub extern crate rbatis_codegen;
extern crate rbatis_macro_driver;
// pub extern crate rbdc;
pub mod crud;
pub mod executor;
pub mod page;
pub mod decode;
#[macro_use]
pub mod impled;

pub use rbatis_macro_driver::{html_sql, py_sql, snake_name, sql};

pub use executor::Executor;
pub use executor::ExecResult;
pub use executor::Error;

pub use decode::*;



