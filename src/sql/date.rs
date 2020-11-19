use serde_json::Value;

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::crud::ColumnFormat;

#[derive(Clone, Debug)]
pub struct DateFormat<'a> {
    pub keys: Vec<&'a str>,
}

impl<'a> DateFormat<'a> {
    fn is_end_with(&'a self, column: &'a str) -> bool {
        for item in &self.keys {
            if column.ends_with(*item) {
                return true;
            }
        }
        return false;
    }
}


impl<'a> ColumnFormat for DateFormat<'a> {
    fn format(&self, driver_type: &DriverType, column: &str, value_sql: &mut String, value: &serde_json::Value) -> crate::core::Result<()> {
        if driver_type.eq(&DriverType::Postgres)
            && !value.is_null()
            && self.is_end_with(column) {
            *value_sql = format!("{}::timestamp", value_sql);
        }
        return Ok(());
    }
}


#[test]
pub fn test_date() {}