/// Simplifies table construction by relying on the Default trait
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
/// need impl Clone or #[derive(Clone, Debug)]
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
            match item.$field_name.as_ref() {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(),item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}