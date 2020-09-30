use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;

use crate::crud::ColumnFormat;

#[derive(Copy, Clone, Debug)]
pub struct DateFormat {}

impl ColumnFormat for DateFormat {
    fn need_format(&self, driver_type: &DriverType, column: &str) -> bool {
        //DateTime only pg need format
        if !driver_type.eq(&DriverType::Postgres) {
            return false;
        }
        if column.contains("date") || column.contains("time") {
            return true;
        }
        return false;
    }

    fn do_format(&self, driver_type: &DriverType, sql: &str, value: &serde_json::Value) -> rbatis_core::Result<(String, Value)> {
        let mut new_sql = String::new();
        match driver_type {
            DriverType::Postgres => {
                new_sql = format!("{}::timestamp", sql);
            }
            _ => {}
        }
        return Ok((new_sql, value.to_owned()));
    }
}


#[test]
pub fn test_date() {}