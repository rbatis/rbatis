/// Simplifies table construction by relying on the Default trait
///
/// step1:  impl Default
/// ```rust
/// use rbatis::table;
/// #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
/// pub struct MockTable{
///    pub id:Option<String>,
///    pub name:Option<String>,
/// }
/// //step2: make struct
/// let activity = table!(MockTable{id : "12312".to_string()});
/// ```
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
/// for example:
///```rust
///use rbatis::table_field_vec;
///struct SysUserRole{
///  pub role_id: Option<String>
///}
///let user_roles: Vec<SysUserRole> = vec![];
///let role_ids_ref: Vec<&String> = table_field_vec!(&user_roles,role_id);
///let role_ids: Vec<String> = table_field_vec!(user_roles,role_id);
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_vec {
    (&$vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let vec = &$vec_ref;
        let mut ids = std::vec::Vec::with_capacity(vec.len());
        for item in vec {
            match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.push(v);
                }
                _ => {}
            }
        }
        ids
    }};
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let vec = $vec_ref;
        let mut ids = std::vec::Vec::with_capacity(vec.len());
        for item in vec {
            match item $(.$field_name)+.to_owned() {
                std::option::Option::Some(v) => {
                    ids.push(v);
                }
                _ => {}
            }
        }
        ids
    }};
}

/// take the target Vec member attribute Vec collection
/// for example:
/// ```rust
///use std::collections::HashSet;
///use rbatis::table_field_set;
///
///pub struct SysUserRole{
///  pub role_id:Option<String>
///}
///let user_roles: Vec<SysUserRole> = vec![];
///let role_ids_ref: HashSet<&String> = table_field_set!(&user_roles,role_id);
///let role_ids: HashSet<String> = table_field_set!(user_roles,role_id);
///```
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_set {
    (&$vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let vec = &$vec_ref;
        let mut ids = std::collections::HashSet::with_capacity(vec.len());
        for item in vec {
             match &item $(.$field_name)+ {
                std::option::Option::Some(v) => {
                    ids.insert(v);
                }
                _ => {}
            }
        }
        ids
    }};
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let vec = $vec_ref;
        let mut ids = std::collections::HashSet::with_capacity(vec.len());
        for item in vec {
             match item $(.$field_name)+.to_owned() {
                std::option::Option::Some(v) => {
                    ids.insert(v);
                }
                _ => {}
            }
        }
        ids
    }};
}

/// Gets the HashMap collection of member attributes of the target Vec
/// for example:
/// ```rust
///use std::collections::HashMap;
///use rbatis::table_field_map;
///
///#[derive(Clone)]
///pub struct SysUserRole{
///   pub role_id: Option<String>
///}
///let user_roles: Vec<SysUserRole> = vec![];
///let role_ids_ref: HashMap<String,&SysUserRole> = table_field_map!(&user_roles,role_id);
///let role_ids: HashMap<String,SysUserRole> = table_field_map!(user_roles,role_id);
///```
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_map {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let vec = $vec_ref;
        let mut ids = std::collections::HashMap::with_capacity(vec.len());
        for item in vec {
              match item $(.$field_name)+ {
                std::option::Option::Some(ref v) => {
                    ids.insert(v.clone(), item);
                }
                _ => {}
            }
        }
        ids
    }};
}

/// Gets the BtreeMap collection of member attributes of the target Vec
/// for example:
/// ```rust
///use std::collections::BTreeMap;
/// use rbatis::table_field_btree;
///
///pub struct SysUserRole{
///   pub role_id: Option<String>
///}
///let user_roles: Vec<SysUserRole>=vec![];
///let role_ids_ref: BTreeMap<String,&SysUserRole> = table_field_btree!(&user_roles,role_id);
///let role_ids_owner: BTreeMap<String,SysUserRole> = table_field_btree!(user_roles,role_id);
///```
///
///
#[allow(unused_macros)]
#[macro_export]
macro_rules! table_field_btree {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::BTreeMap::new();
        for item in $vec_ref {
             match &item $(.$field_name)+ {
                std::option::Option::Some(ref v) => {
                    ids.insert(v.clone(), item);
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
///```rust
///pub struct MockTable{
///  pub id:String
///}
///let name=rbatis::field_name!(MockTable.id);
/// ```
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
///```rust
///pub struct MockTable{
///  pub id:String
///}
///let name = rbatis::field_key!(MockTable::id);
/// ```
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


#[deprecated(note = "please use table_field_btree!")]
#[macro_export]
macro_rules! make_table_field_map_btree {
    ($vec_ref:expr,$($field_name:ident$(.)?)+) => {{
        let mut ids = std::collections::BTreeMap::new();
        for item in $vec_ref {
            match item.$($field_name.)+as_ref() {
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
            match item.$($field_name.)+as_ref() {
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
            match item.$($field_name.)+as_ref() {
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
