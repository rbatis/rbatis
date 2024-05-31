/// Simplifies table construction by relying on the Default trait
///
/// step1:  impl Default
/// #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
/// MockTable{
/// }
///
/// //step2: make struct
/// let activity = rbatis::table!(MockTable{
///             id : "12312".to_string(),
///             delete_flag : 1,
///             name:  None,
///             });
#[macro_export]
macro_rules! table {
        ($t:path{ $($key:ident:$value:expr$(,)?)+ }) => {
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
///      let role_ids:Vec<String> = table_field_vec!(&user_roles,role_id);
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_vec {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::vec::Vec::with_capacity($vec_ref.len());
        for item in $vec_ref {
            match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.push(v.clone());
                }
                _ => {}
            }
        }
        ids
    }};
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
///      let role_ids:HashSet<String> = table_field_vec!(&user_roles,role_id);
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_set {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::HashSet::with_capacity($vec_ref.len());
        for item in $vec_ref {
             match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone());
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
///      let role_ids: HashMap<String,SysUserRole> = table_field_map!(&user_roles,role_id);
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_map {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::HashMap::with_capacity($vec_ref.len());
        for item in $vec_ref {
              match &item $(.$field_name)+ {
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
///      let role_ids = table_field_map_btree!(&user_roles,role_id); // role_ids: HashMap<String,SysUserRole>
///
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_map_btree {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::BTreeMap::new();
        for item in $vec_ref {
             match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(), item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

/// Used to simulate enumerations to improve code maintainability.
/// this is return &str data
/// for example:
/// let name=field_name!(MockTable.id);
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! field_name {
    ($t:ident.$field:ident) => {{
        if false {
            let _ = |a: $t| a.$field;
        }
        stringify!($field).trim_start_matches("r#")
    }};
    ($t:ident.$field1:ident.$field2:ident) => {{
        if false {
            let _ = |a: $t| a.$field1.$field2;
        }
        stringify!($field2).trim_start_matches("r#")
    }};
    ($t:ident.$field1:ident.$field2:ident.$field3:ident) => {{
        if false {
            let _ = |a: $t| a.$field1.$field2.$field3;
        }
        stringify!($field3).trim_start_matches("r#")
    }};
}

/// Used to simulate enumerations to improve code maintainability.
/// this is return &str data
/// for example:
/// let name=field_key!(MockTable::id);
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! field_key {
    ($t:ident::$field:ident) => {{
        if false {
            let _ = |a: $t| a.$field;
        }
        stringify!($field).trim_start_matches("r#")
    }};
    ($t:ident::$field1:ident::$field2:ident) => {{
        if false {
            let _ = |a: $t| a.$field1.$field2;
        }
        stringify!($field2).trim_start_matches("r#")
    }};
    ($t:ident::$field1:ident::$field2:ident::$field3:ident) => {{
        if false {
            let _ = |a: $t| a.$field1.$field2.$field3;
        }
        stringify!($field3).trim_start_matches("r#")
    }};
}




#[deprecated(note = "please use table_field_map_btree!")]
#[macro_export]
macro_rules! make_table_field_map_btree {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::BTreeMap::new();
        for item in $vec_ref {
             match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(), item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

#[deprecated(note = "please use table_field_map!")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_table_field_map {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::HashMap::with_capacity($vec_ref.len());
        for item in $vec_ref {
              match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.insert(v.clone(), item.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

#[deprecated(note = "please use table_field_vec!")]
#[allow(unused_macros)]
#[macro_export]
macro_rules! make_table_field_vec {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::vec::Vec::with_capacity($vec_ref.len());
        for item in $vec_ref {
            match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.push(v.clone());
                }
                _ => {}
            }
        }
        ids
    }};
}

#[deprecated(note = "please use table!")]
#[macro_export]
macro_rules! make_table {
        ($t:path{ $($key:ident:$value:expr$(,)?)+ }) => {
           {
            let mut temp_table_data = <$t>::default();
            $(temp_table_data.$key = $value.into();)+
            temp_table_data
           }
        }
}
