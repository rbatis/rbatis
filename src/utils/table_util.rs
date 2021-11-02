use std::collections::{BTreeMap, HashMap};

use crate::crud::CRUDTable;


/// Simplifies table construction by relying on the Default trait
///
/// step1:  impl Default
/// #[crud_table]
/// #[derive(Clone, Debug, Default)]
/// BizActivity{
/// }
///
/// //step2: make struct
/// let activity = rbatis::make_table!(BizActivity{
///             id : "12312".to_string(),
///             delete_flag : 1,
///             name:  None,
///             });
#[macro_export]
macro_rules! make_table {
        ($t:ty{ $($key:ident:$value:expr$(,)?)+ }) => {
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
/// need impl Clone or #[derive(Clone, Debug)]
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_table_field_vec!(&user_roles,role_id); // role_ids: Vec<String>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_table_field_vec {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = vec![];
        for item in $vec_ref {
            match item.$field_name.as_ref() {
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
/// need impl Clone or #[derive(Clone, Debug)]
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_table_field_map!(&user_roles,role_id); // role_ids: HashMap<String,SysUserRole>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_table_field_map {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = std::collections::HashMap::new();
        for item in $vec_ref {
            match item.$field_name.as_ref() {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(), item.clone());
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
/// need impl Clone or #[derive(Clone, Debug)]
/// for example:
///      struct SysUserRole{
///         pub role_id:String
///      }
///      let user_roles: Vec<SysUserRole>;
///      let role_ids = make_table_field_map_btree!(&user_roles,role_id); // role_ids: HashMap<String,SysUserRole>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_table_field_map_btree {
    ($vec_ref:expr,$field_name:ident) => {{
        let mut ids = std::collections::BTreeMap::new();
        for item in $vec_ref {
            match item.$field_name.as_ref() {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(), item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}


/**
 this macro allow gen column fn
  #[crud_table]
  #[derive(Clone, Debug)]
  make_column_fn!(
    pub struct BizActivity {
       pub id: Option<String>
   }
 );
println!("{}",BizActivity::id());
 **/
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_column_fn {
    (pub struct $t:ty{ $(pub $key:ident:$value:expr$(,)?)+ }) => {
           pub struct $t{
                   $(pub $key:$value,)+
           }
           impl $t{
                       $(
                         #[inline]
                         pub fn $key()->&'static str{
                             stringify!($key)
                         }
                       )+
               }
        }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! as_bson {
    ($key:expr) => {
        bson::to_bson($key).unwrap_or_default()
    }
}

/// Used to simulate enumerations to improve code maintainability
/// for example:
/// rb.new_wrapper().eq(column_name!(BizActivity::id),"1")
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! column_name {
    ($t:ident::$field:ident) => {
       if true{
           stringify!($field).trim_start_matches("r#")
       }else{
           {
               //此处代码伪造引用，误导idea 关联类型$t和 $field,实现智能提示
               let f=|a:$t|{let c=a.$field;};
               ""
           }
       }
    };
}