use rbatis_core::db::DriverType;

use crate::lang::PageLimit;

impl PageLimit for DriverType {
    fn create(&self, offset: i64, size: i64) -> rbatis_core::Result<String> {
        return match self {
            DriverType::Mysql => {
                Ok(format!(" LIMIT {},{}", offset, size))
            }
            DriverType::Postgres => {
                Ok(format!(" LIMIT {} OFFSET {}", size, offset))
            }
            DriverType::Sqlite => {
                Ok(format!(" LIMIT {} OFFSET {}", size, offset))
            }
            _ => {
                Err(rbatis_core::Error::from(format!("[rbatis] not support now for DriverType:{:?}", self)))
            }
        };
    }
}

#[test]
pub fn test_create_limit() {
    let mysql_limit = DriverType::Mysql.create(1, 20).unwrap();
    println!("{}", mysql_limit);
    let pg_limit = DriverType::Postgres.create(1, 20).unwrap();
    println!("{}", pg_limit);
    let sqlite_limit = DriverType::Sqlite.create(1, 20).unwrap();
    println!("{}", sqlite_limit);
}