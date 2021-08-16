use crate::crud::{CRUDTable, Skip};
use crate::DriverType;

use bson::Bson;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

pub trait VersionLockPlugin: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    /// database column must be i32 or i64 or time column!
    fn column(&self) -> &str;

    /// set value = value + 1, support number and string value
    fn try_add_one(&self, old_value: &Bson, column: &str) -> Bson {
        if self.column().eq(column) {
            match old_value {
                Bson::String(s) => {
                    let version = s.parse::<i64>();
                    match version {
                        Ok(version) => {
                            return Bson::String((version + 1).to_string());
                        }
                        _ => {}
                    }
                }

                Bson::Int32(m) => {
                    return Bson::Int32(m + 1);
                }

                Bson::Int64(m) => {
                    return Bson::Int64(m + 1);
                }

                _ => {}
            }
        }
        return old_value.clone();
    }

    fn try_make_where_sql(&self, old_version: &Bson) -> String {
        if !old_version.eq(&Bson::Null) {
            format!("{} = {} ", self.column(), old_version)
        } else {
            return String::default();
        }
    }
}

#[derive(Debug, Clone)]
pub struct RbatisVersionLockPlugin {
    pub excludes: Vec<String>,
    pub version_column: String,
}

impl RbatisVersionLockPlugin {
    pub fn new(version_column: &str) -> Self {
        Self {
            excludes: vec![],
            version_column: version_column.to_owned(),
        }
    }
}

impl VersionLockPlugin for RbatisVersionLockPlugin {
    fn column(&self) -> &str {
        &self.version_column
    }
}

/// use this context will not use logic del
pub struct TableNoVersion<T>
where
    T: CRUDTable,
{
    pub table: T,
}

impl<T> Serialize for TableNoVersion<T>
where
    T: CRUDTable,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        T::serialize(&self.table, serializer)
    }
}

impl<'de, T> Deserialize<'de> for TableNoVersion<T>
where
    T: CRUDTable,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let result = T::deserialize(deserializer)?;
        return Ok(TableNoVersion { table: result });
    }
}

impl<T> CRUDTable for TableNoVersion<T>
where
    T: CRUDTable,
{
    fn is_use_plugin(plugin_name: &str) -> bool {
        if plugin_name.eq(std::any::type_name::<RbatisVersionLockPlugin>()) {
            return false;
        }
        return true;
    }

    fn table_name() -> String {
        T::table_name()
    }

    fn table_columns() -> String {
        T::table_columns()
    }

    fn formats(driver_type: &DriverType) -> HashMap<String, fn(arg: &str) -> String> {
        T::formats(driver_type)
    }

    fn make_value_sql_arg(
        &self,
        db_type: &DriverType,
        index: &mut usize,
        skips: &[Skip],
    ) -> crate::Result<(String, String, Vec<Bson>)> {
        T::make_value_sql_arg(&self.table, db_type, index, skips)
    }
}

impl<T> Deref for TableNoVersion<T>
where
    T: CRUDTable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<T> From<T> for TableNoVersion<T>
where
    T: CRUDTable,
{
    fn from(arg: T) -> Self {
        TableNoVersion { table: arg }
    }
}
