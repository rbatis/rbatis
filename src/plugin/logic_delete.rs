use serde_json::Value;
use std::fmt::{Debug, Display};

use crate::core::db::DriverType;
use crate::core::Error;
use crate::sql::upper::SqlReplaceCase;

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
    /// create_select_sql
    fn create_select_sql(
        &self,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        sql_where: &str,
    ) -> Result<String, crate::core::Error>;
}

#[derive(Debug)]
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
            let new_sql = format!(
                "UPDATE {} SET {} = {}",
                table_name,
                self.column(),
                self.deleted()
            ) + sql_where;
            Ok(new_sql)
        } else if !sql_where.is_empty() {
            let new_sql = format!("DELETE FROM {} {}", table_name, sql_where.trim_start());
            Ok(new_sql)
        } else {
            Err(Error::from("[rbatis] del data must have where sql!"))
        };
    }

    fn create_select_sql(
        &self,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        where_sql: &str,
    ) -> Result<String, Error> {
        let mut where_sql = where_sql
            .replace(" order by ", " ORDER BY ")
            .replace(" group by ", " GROUP BY ")
            .trim()
            .to_string();
        let mut sql = String::new();
        if table_fields.contains(self.column()) {
            if where_sql.is_empty() {
                where_sql = format!("{} = {}", self.column(), self.un_deleted());
            } else {
                if where_sql.starts_with("ORDER BY") || where_sql.starts_with("GROUP BY") {
                    where_sql =
                        format!("{} = {} ", self.column(), self.un_deleted()) + where_sql.as_str();
                } else {
                    where_sql = format!("{} = {} AND ", self.column(), self.un_deleted())
                        + where_sql.as_str();
                }
            }
        }
        sql = format!(
            "SELECT {} FROM {} {}",
            table_fields, table_name, driver_type.try_add_where_sql(&where_sql)
        );
        Ok(sql)
    }
}
