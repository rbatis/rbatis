use serde_json::Number;
use std::fmt::{Debug, Formatter};
use crate::crud::{CRUDTable, Skip};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::DriverType;
use std::collections::HashMap;
use std::ops::Deref;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub trait VersionLockPlugin: Send + Sync + Debug {

    ///the name
    fn table_name(&self) -> String;

    /// database column must be i32 or i64 or time column!
    fn column(&self) -> &str;

    /// set value = value + 1, support number and string value
    fn try_add_one(
        &self,
        old_value: &serde_json::Value,
        column: &str,
    ) -> serde_json::Value {
        if self.column().eq(column) {
            match old_value {
                serde_json::Value::String(s) => {
                    let version = s.parse::<i64>();
                    match version {
                        Ok(version) => {
                            return serde_json::Value::String((version + 1).to_string());
                        }
                        _ => {}
                    }
                }
                serde_json::Value::Number(n) => {
                    if n.is_i64() {
                        return serde_json::json!(n.as_i64().unwrap_or(0) + 1);
                    } else if n.is_u64() {
                        return serde_json::json!(n.as_u64().unwrap_or(0) + 1);
                    }
                }
                _ => {}
            }
        }
        return old_value.clone();
    }

    fn try_make_where_sql(&self, old_version: &serde_json::Value) -> String {
        if !old_version.eq(&serde_json::Value::Null) {
            format!("{} = {} ", self.column(), old_version)
        } else {
            return String::default();
        }
    }
}

#[derive(Clone)]
pub struct RbatisVersionLockPlugin<T> where T:CRUDTable{
    pub excludes: Vec<String>,
    pub version_column: String,
    pub table:PhantomData<T>
}

impl <T>RbatisVersionLockPlugin<T> where T:CRUDTable{
    pub fn new(version_column: &str) -> Self {
        Self {
            excludes: vec![],
            version_column: version_column.to_owned(),
            table: Default::default()
        }
    }

    pub fn table_name() -> String {
        T::table_name()
    }

}

impl<T> Debug for RbatisVersionLockPlugin<T> where T: CRUDTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RbatisVersionLockPlugin")
            .finish()
    }
}

impl <T>VersionLockPlugin for RbatisVersionLockPlugin<T> where T:CRUDTable{
    fn table_name(&self) -> String {
        RbatisVersionLockPlugin::<T>::table_name()
    }

    fn column(&self) -> &str {
        &self.version_column
    }
}
