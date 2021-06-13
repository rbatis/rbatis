#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

mod crud_test;
mod dyn_table_test;
mod format_test;
mod ids_test;
mod intercept_test;
mod page_test;
mod plugin_exclude_test;
mod py_sql_test;
mod html_sql_test;
mod raw_driver_test;
mod raw_identifiers_test;
mod raw_sql_macro_test;
mod table_macro_test;
mod transaction_test;
mod un_support_type_test;
mod wrapper_test;

use chrono::NaiveDateTime;
/// this is table model(see ../database.sql)
#[crud_table]
#[derive(Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}
