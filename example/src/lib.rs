#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;

mod model;
mod crud_test;
mod un_support_type_test;
mod raw_driver_test;
mod wrapper_test;
mod page_test;
mod format_test;
mod custom_py_sql_test;
mod macro_test;
mod raw_identifiers_test;
mod transaction_test;

use model::BizActivity;