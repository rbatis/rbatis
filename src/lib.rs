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
#[macro_export]
macro_rules! table {
         ($t:ty{ $($key:ident:$value:expr,)+ }) => {
           {
            let mut temp_table_data = <$t>::default();
            $(temp_table_data.$key=$value;)+
            temp_table_data
           }
        }
}
/// Simplifies table construction by relying on the Default interface
///
/// //step1:  impl Default
/// impl Default for BizActivity{
///       fn default() -> Self {
///          Self{
///            id:None,
///            name:None,
///            delete_flag:None,
///          }
///      }
/// }
///  //step2: make struct
/// let activity = rbatis::table_some!(BizActivity{
///             id : "12312".to_string(),
///             delete_flag : 1,
///             });
#[macro_export]
macro_rules! table_some {
         ($t:ty{ $($key:ident:$value:expr,)+ }) => {
           {
            let mut temp_table_data = <$t>::default();
            $(temp_table_data.$key=Some($value);)+
            temp_table_data
           }
        }
}