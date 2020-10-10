use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;

use crate::crud::ColumnFormat;

#[derive(Copy, Clone, Debug)]
pub struct DateFormat {}

impl ColumnFormat for DateFormat {
    fn format(&self, driver_type: &DriverType, column: &str) -> rbatis_core::Result<String> {
        let mut new_sql = column.to_string();
        if driver_type.eq(&DriverType::Postgres) && (
            column.ends_with("date") || column.ends_with("time")
                || column.ends_with("Date") || column.ends_with("Time")) {
            new_sql = format!("{}::timestamp", column);
        }
        return Ok(new_sql);
    }
}


#[test]
pub fn test_date() {}