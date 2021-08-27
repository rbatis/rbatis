use std::fmt::{Debug, Display, Formatter};

use serde_json::Value;

use crate::core::db::DriverType;
use crate::core::Error;
use crate::sql::rule::SqlRule;
use crate::crud::{CRUDTable, Skip};
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// Logic Delete Plugin trait
pub trait LogicDelete: Send + Sync + Debug {
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

pub struct RbatisLogicDeletePlugin {
    pub column: String,
    pub deleted: i32,
    pub un_deleted: i32,
}

impl RbatisLogicDeletePlugin {
    pub fn new(column: &str) -> Self {
        Self {
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
            column: column.to_string(),
            deleted,
            un_deleted,
        }
    }
}

impl Debug for RbatisLogicDeletePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RbatisLogicDeletePlugin")
            .finish()
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