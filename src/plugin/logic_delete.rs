use std::fmt::{Debug, Display};

use serde_json::Value;

use crate::core::db::DriverType;
use crate::core::Error;
use crate::sql::rule::SqlRule;

/// Logic Delete Plugin trait
pub trait LogicDelete: Send + Sync + Debug {
    ///the name
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_allow(&self, context_id: &str) -> bool;

    /// database column
    fn column(&self) -> &str;
    /// deleted data,must be i32
    fn deleted(&self) -> i32;
    /// un deleted data,must be i32
    fn un_deleted(&self) -> i32;
    /// create_remove_sql
    fn create_remove_sql(
        &self,
        context_id: &str,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        sql_where: &str,
    ) -> Result<String, crate::core::Error>;
    /// create_select_sql
    fn create_select_sql(
        &self,
        context_id: &str,
        driver_type: &DriverType,
        table_name: &str,
        column: &str,
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
    fn is_allow(&self, context_id: &str) -> bool {
        for x in &self.excludes {
            if context_id.starts_with(x) {
                return false;
            }
        }
        return true;
    }

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
        context_id: &str,
        driver_type: &DriverType,
        table_name: &str,
        table_fields: &str,
        sql_where: &str,
    ) -> Result<String, Error> {
        if !self.is_allow(context_id) {
            //make delete sql
            let new_sql = format!(
                "{} {} {}",
                crate::sql::TEMPLATE.delete_from.value,
                table_name,
                sql_where.trim_start()
            );
            return Ok(new_sql);
        }
        return if table_fields.contains(self.column()) {
            //fields have column
            let new_sql = format!(
                "{} {} {} {} = {}",
                crate::sql::TEMPLATE.update.value,
                table_name,
                crate::sql::TEMPLATE.set.value,
                self.column(),
                self.deleted()
            ) + sql_where;
            Ok(new_sql)
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

    fn create_select_sql(
        &self,
        context_id: &str,
        driver_type: &DriverType,
        table_name: &str,
        column: &str,
        table_fields: &str,
        where_sql: &str,
    ) -> Result<String, Error> {
        let mut where_sql = where_sql.trim().to_string();
        let mut sql = String::new();
        if self.is_allow(context_id) && table_fields.contains(self.column()) {
            where_sql = driver_type.make_left_insert_where(
                &format!("{} = {}", self.column(), self.un_deleted()),
                &where_sql,
            );
        }
        sql = format!(
            "{} {} {} {} {}",
            crate::sql::TEMPLATE.select.value,
            column,
            crate::sql::TEMPLATE.from.value,
            table_name,
            driver_type.make_where(&where_sql)
        );
        Ok(sql)
    }
}
