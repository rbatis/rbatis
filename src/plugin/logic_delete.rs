//TODO delete_flag plugin


use rbatis_core::db::DriverType;
use serde_json::Value;
use rbatis_core::Error;

pub trait LogicDelete: Send + Sync {
    fn column(&self) -> &str;
    fn deleted(&self) -> i32;
    fn un_deleted(&self) -> i32;
    fn make_delete_sql(&self, driver_type: &DriverType, table_name: &str, sql_where: &str) -> Result<String, rbatis_core::Error>;
}


pub struct RbatisLogicDeletePlugin {
    pub column: String
}

impl LogicDelete for RbatisLogicDeletePlugin {
    fn column(&self) -> &str {
        self.column.as_str()
    }

    fn deleted(&self) -> i32 {
        0
    }

    fn un_deleted(&self) -> i32 {
        1
    }


    fn make_delete_sql(&self, driver_type: &DriverType, table_name: &str, sql_where: &str) -> Result<String, Error> {
        let mut new_sql = format!("UPDATE {} SET {} = {}", table_name, self.column(), self.deleted()) + sql_where;
        return Ok(new_sql);
    }
}