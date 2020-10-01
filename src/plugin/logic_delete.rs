use serde_json::Value;

use rbatis_core::db::DriverType;
use rbatis_core::Error;

/// Logic Delete Plugin trait
pub trait LogicDelete: Send + Sync {
    /// database column
    fn column(&self) -> &str;
    /// deleted data,must be i32
    fn deleted(&self) -> i32;
    /// un deleted data,must be i32
    fn un_deleted(&self) -> i32;
    /// create_remove_sql
    fn create_remove_sql(&self, driver_type: &DriverType, table_name: &str, table_fields: &str, sql_where: &str) -> Result<String, rbatis_core::Error>;
    /// create_select_sql
    fn create_select_sql(&self, driver_type: &DriverType, table_name: &str, table_fields: &str, sql_where: &str) -> Result<String, rbatis_core::Error>;
}

pub enum LogicAction {
    Select,
    Delete,
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


    fn create_remove_sql(&self, driver_type: &DriverType, table_name: &str, table_fields: &str, sql_where: &str) -> Result<String, Error> {
        return if table_fields.contains(self.column()) {
            //fields have column
            let new_sql = format!("UPDATE {} SET {} = {}", table_name, self.column(), self.deleted()) + sql_where;
            Ok(new_sql)
        } else if !sql_where.is_empty() {
            let new_sql = format!("DELETE FROM {} {}", table_name, sql_where.trim_start());
            Ok(new_sql)
        } else {
            Err(Error::from("[rbatis] del data must have where sql!"))
        };
    }

    fn create_select_sql(&self, driver_type: &DriverType, table_name: &str, table_fields: &str, where_sql: &str) -> Result<String, Error> {
        let mut where_sql = where_sql.trim().to_string();
        let mut sql = String::new();
        if table_fields.contains(self.column()) {
            if where_sql.is_empty() {
                where_sql = format!("{} = {}", self.column(), self.un_deleted());
            } else {
                if where_sql.starts_with("ORDER BY") || where_sql.starts_with("GROUP BY") {
                    where_sql = format!("{} = {} ", self.column(), self.un_deleted()) + where_sql.as_str();
                } else {
                    where_sql = format!("{} = {} AND ", self.column(), self.un_deleted()) + where_sql.as_str();
                }
            }
        }
        if !where_sql.is_empty() {
            sql = format!("SELECT {} FROM {} WHERE {}", table_fields, table_name, where_sql);
        } else {
            sql = format!("SELECT {} FROM {}", table_fields, table_name);
        }
        Ok(sql)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_delete_plugin_create_sql() {
        let r = RbatisLogicDeletePlugin::new("del");
        let table_fields = "name,age,del".to_string();
        let sql_where = "";
        let result = r.create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where).unwrap();
        assert_eq!("UPDATE test SET del = 1", &result);

        let sql_where = " WHERE name = 'zhangsan'";
        let result = r.create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where).unwrap();
        assert_eq!("UPDATE test SET del = 1 WHERE name = 'zhangsan'", &result);

        let table_fields = "name,age";
        let sql_where = " WHERE name = 'zhangsan'";
        let result = r.create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where).unwrap();
        assert_eq!("DELETE FROM test WHERE name = 'zhangsan'", &result);

        let table_fields = "name,age";
        let sql_where = "";
        let result = r.create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where);
        assert!(result.is_err());
    }
}