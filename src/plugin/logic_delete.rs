use std::fmt::{Debug, Display};

use serde_json::Value;

use crate::core::db::DriverType;
use crate::core::Error;
use crate::sql::rule::SqlRule;
use crate::crud::{CRUDTable, Skip};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// Logic Delete Plugin trait
pub trait LogicDelete: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// database column
    fn column(&self) -> &str;
    /// deleted data,must be i32
    fn deleted(&self) -> i32;
    /// un deleted data,must be i32
    fn un_deleted(&self) -> i32;
    /// create_remove_sql
    fn create_remove_sql(
        &self,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        sql_where: &str,
    ) -> Result<String, crate::core::Error>;
}

#[derive(Debug)]
pub struct RbatisLogicDeletePlugin {
    pub excludes: Vec<String>,
    pub column: String,
    pub deleted: i32,
    pub un_deleted: i32,
}

impl RbatisLogicDeletePlugin {
    pub fn new(column: &str) -> Self {
        Self {
            excludes: vec![],
            column: column.to_string(),
            deleted: 1,
            un_deleted: 0,
        }
    }
    pub fn new_opt(column: &str, deleted: i32, un_deleted: i32) -> Self {
        if deleted == un_deleted {
            panic!("[rbaits] deleted can not equal to un_deleted on RbatisLogicDeletePlugin::new_opt(column: &str, deleted: i32, un_deleted: i32)")
        }
        Self {
            excludes: vec![],
            column: column.to_string(),
            deleted,
            un_deleted,
        }
    }
}

impl LogicDelete for RbatisLogicDeletePlugin {
    fn column(&self) -> &str {
        self.column.as_str()
    }

    fn deleted(&self) -> i32 {
        self.deleted
    }

    fn un_deleted(&self) -> i32 {
        self.un_deleted
    }

    fn create_remove_sql(
        &self,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        sql_where: &str,
    ) -> Result<String, Error> {
        return if table_fields.contains(self.column()) {
            //fields have column
            if sql_where.is_empty() {
                let new_sql = format!(
                    "{} {} {} {} = {}",
                    crate::sql::TEMPLATE.update.value,
                    table_name,
                    crate::sql::TEMPLATE.set.value,
                    self.column(),
                    self.deleted()
                ) + sql_where;
                Ok(new_sql)
            } else {
                let new_sql = format!(
                    "{} {} {} {} = {} {}",
                    crate::sql::TEMPLATE.update.value,
                    table_name,
                    crate::sql::TEMPLATE.set.value,
                    self.column(),
                    self.deleted(),
                    sql_where.trim_start()
                );
                Ok(new_sql)
            }
        } else if !sql_where.is_empty() {
            let new_sql = format!(
                "{} {} {}",
                crate::sql::TEMPLATE.delete_from.value,
                table_name,
                sql_where.trim_start()
            );
            Ok(new_sql)
        } else {
            Err(Error::from("[rbatis] del data must have where sql!"))
        };
    }
}


/// use this context will not use logic del
pub struct TableNoLogic<T> where T: CRUDTable {
    pub table: T,
}

impl<T> Serialize for TableNoLogic<T> where T: CRUDTable {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        T::serialize(&self.table, serializer)
    }
}

impl<'de, T> Deserialize<'de> for TableNoLogic<T> where T: CRUDTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let result = T::deserialize(deserializer)?;
        return Ok(TableNoLogic {
            table: result,
        });
    }
}


impl<T> CRUDTable for TableNoLogic<T> where T: CRUDTable {
    fn is_use_plugin(plugin_name: &str) -> bool {
        if plugin_name.eq(std::any::type_name::<RbatisLogicDeletePlugin>()) {
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
        skips: &[Skip]
    ) -> crate::Result<(String, String, Vec<serde_json::Value>)> {
        T::make_value_sql_arg(&self.table, db_type, index,skips)
    }
}

impl<T> Deref for TableNoLogic<T> where T: CRUDTable {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<T> DerefMut for TableNoLogic<T> where T: CRUDTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<T> From<T> for TableNoLogic<T> where T: CRUDTable {
    fn from(arg: T) -> Self {
        TableNoLogic {
            table: arg
        }
    }
}
