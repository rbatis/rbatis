use rbatis_core::db::DriverType;

use crate::sql::PageLimit;

impl PageLimit for DriverType {
    fn page_limit_sql(&self, offset: u64, size: u64) -> rbatis_core::Result<String> {
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
            DriverType::Mssql => {
                //sqlserver
                Ok(format!(" OFFSET {} ROWS FETCH NEXT {} ROWS ONLY", offset,size))
            }
            DriverType::None => {
                Err(rbatis_core::Error::from(format!("[rbatis] not support now for DriverType:{:?}", DriverType::None)))
            }
        };
    }
}

#[test]
pub fn test_create_limit() {
    let mysql_limit = DriverType::Mysql.page_limit_sql(1, 20).unwrap();
    println!("{}", mysql_limit);
    let pg_limit = DriverType::Postgres.page_limit_sql(1, 20).unwrap();
    println!("{}", pg_limit);
    let sqlite_limit = DriverType::Sqlite.page_limit_sql(1, 20).unwrap();
    println!("{}", sqlite_limit);
}