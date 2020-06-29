use rbatis_core::db::DriverType;

use crate::lang::PageLimit;

impl PageLimit for DriverType {
    fn create(&self, offset: i64, size: i64) -> rbatis_core::Result<String> {
        return match self {
            DriverType::Mysql => {
                Ok(format!(" LIMIT {},{}", offset, size))
            }
            DriverType::Postgres => {
                Ok(format!(" LIMIT {} offset {}", size, offset))
            }
            DriverType::Sqlite => {
                Ok(format!(" LIMIT {} offset {}", size, offset))
            }
            _ => {
                Err(rbatis_core::Error::from(format!("[rbatis] not support now for DriverType:{:?}", self)))
            }
        };
    }
}