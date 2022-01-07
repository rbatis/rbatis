#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;

mod crud;
mod dyn_table_name;
mod format;
mod get_fields;
mod plugin_intercept;
mod plugin_page;
mod plugin_logic_del;
mod macro_proc_pysql;
mod macro_proc_htmlsql;
mod macro_proc_rawsql;
mod raw_driver;
mod column_keyword;
mod macro_tool;
mod transaction;
mod plugin_snowflake;
mod plugin_object_id;
mod wrapper;
mod wrapper_macro;



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
    pub create_time: Option<rbatis::DateTimeNative>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}
