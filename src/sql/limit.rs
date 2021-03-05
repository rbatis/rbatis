use crate::core::db::DriverType;

pub trait PageLimit {
    /// return  sql
    fn page_limit_sql(&self, offset: u64, size: u64) -> crate::core::Result<String>;
}

impl PageLimit for DriverType {
    fn page_limit_sql(&self, offset: u64, size: u64) -> crate::core::Result<String> {
        return match self {
            DriverType::Mysql => Ok(format!(" {} {},{}", crate::sql::TEMPLATE.limit, offset, size)),
            DriverType::Postgres => Ok(format!(" {} {} {} {}", crate::sql::TEMPLATE.limit, size, crate::sql::TEMPLATE.offset, offset)),
            DriverType::Sqlite => Ok(format!(" {} {} {} {}", crate::sql::TEMPLATE.limit, size, crate::sql::TEMPLATE.offset, offset)),
            DriverType::Mssql => {
                //sqlserver
                Ok(format!(
                    " {} {} {} {} {}",
                    crate::sql::TEMPLATE.offset,
                    offset,
                    crate::sql::TEMPLATE.rows_fetch_next,
                    size,
                    crate::sql::TEMPLATE.rows_only
                ))
            }
            DriverType::None => Err(crate::core::Error::from(format!(
                "[rbatis] not support now for DriverType:{:?}",
                DriverType::None
            ))),
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
