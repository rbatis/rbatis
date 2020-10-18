use serde_json::Value;

use rbatis_core::convert::StmtConvert;
use rbatis_core::db::DriverType;

use crate::crud::ColumnFormat;

#[derive(Copy, Clone, Debug)]
pub struct DateFormat {}

impl ColumnFormat for DateFormat {
    fn format(&self, driver_type: &DriverType, column: &str, value_sql: &mut String, value: &serde_json::Value) -> rbatis_core::Result<()> {
        if driver_type.eq(&DriverType::Postgres)
            && !value.is_null()
            && (column.ends_with("date") || column.ends_with("time") || column.ends_with("Date") || column.ends_with("Time")) {
            *value_sql = format!("{}::timestamp", value_sql);
        }
        return Ok(());
    }
}


#[test]
pub fn test_date() {}