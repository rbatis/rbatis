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
extern crate serde_json;

pub use rbatis_core as core;

pub use rbatis_macro_driver::{crud_enable, CRUDTable, py_sql, sql};

pub use crate::core::{convert::StmtConvert, db::DriverType, error::Error, error::Result};

pub mod crud;
pub mod plugin;
pub mod rbatis;
pub mod sql;
pub mod tx;
#[macro_use]
pub mod utils;
pub mod wrapper;


/// Simplifies table construction by relying on the Default interface
///
/// step1:  impl Default
/// impl Default for BizActivity{
///       fn default() -> Self {
///          Self{
///            id:None,
///            name:None,
///            delete_flag:None,
///          }
///      }
/// }
/// //step2: make struct
/// let activity = rbatis::table!(BizActivity{
///             id : Some("12312".to_string()),
///             delete_flag : Some(1),
///             });
/// //or use into trait
/// let activity = rbatis::table!(BizActivity{
///             id : "12312".to_string(),
///             delete_flag : 1,
///             });
#[macro_export]
macro_rules! table {
         ($t:ty{ $($key:ident:$value:expr,)+ }) => {
           {
            let mut temp_table_data = <$t>::default();
            $(temp_table_data.$key = $value.into();)+
            temp_table_data
           }
        }
}
/// take the target Vec member attribute Vec collection
/// vec_ref: a reference to vec, field_name: the field name of the structure
///
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = table_field_vec!(&user_roles,role_id); // role_ids: Vec<String>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_vec {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = vec![];
        for item in $vec_ref {
            match &item.$field_name {
                std::option::Option::Some(v) => {
                    ids.push(v.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

/// Gets the HashMap collection of member attributes of the target Vec
/// vec_ref: vec reference，field_name: the field name of the structure
///
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = table_field_map!(&user_roles,role_id); // role_ids: HashMap<String,SysUserRole>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_map {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = std::collections::HashMap::new();
        for item in $vec_ref {
            match &item.$field_name {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(),item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}