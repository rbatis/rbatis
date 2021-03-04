use std::collections::HashMap;

use crate::crud::CRUDTable;

/// Father-Child Relationship
pub trait FatherChildRelationship where Self: CRUDTable + Clone {
    fn get_father_id(&self) -> Option<&Self::IdType>;
    fn set_childs(&mut self, arg: Vec<Self>);
    ///recursive_set_childs for Relationship
    fn recursive_set_childs(&mut self, all_record: &HashMap<Self::IdType, Self>) {
        let mut childs: Option<Vec<Self>> = None;
        if self.get_id().is_some() {
            for (key, x) in all_record {
                if x.get_father_id().is_some() && self.get_id().eq(&x.get_father_id()) {
                    let mut item = x.clone();
                    item.ecursive_set_childs(all_record);
                    match &mut childs {
                        Some(childs) => {
                            childs.push(item);
                        }
                        None => {
                            let mut vec = vec![];
                            vec.push(item);
                            childs = Some(vec);
                        }
                    }
                }
            }
        }
        if childs.is_some() {
            self.set_childs(childs.unwrap())
        }
    }
}



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
/// let activity = rbatis::make_table!(BizActivity{
///             id : Some("12312".to_string()),
///             delete_flag : Some(1),
///             });
/// //or use into trait
/// let activity = rbatis::make_table!(BizActivity{
///             id : "12312".to_string(),
///             delete_flag : 1,
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
                    ids.insert(v.clone(),item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}