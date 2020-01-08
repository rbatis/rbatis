use std::error::Error;
use std::sync::Arc;

use log::{error, info, warn};
use rdbc::{Connection, Driver};
use rdbc_mysql::MySQLDriver;
use rdbc_postgres::PostgresDriver;

use crate::core::db_config::DBConfig;

pub fn get_conn(arg: &DBConfig) -> Result<Box<dyn Connection>, String> {
    let link = arg.to_string();
    println!("link:{}",link);
    if arg.db_type.eq("mysql") {
        let driver = Arc::new(MySQLDriver::new());
        let conn = driver.connect(link.as_str());
        if conn.is_err() {
            let info = "[rbatis] connect mysql server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(info);
        }
        return Result::Ok(conn.unwrap());
    } else if arg.db_type.eq("postgres") {
        let driver: Arc<dyn rdbc::Driver> = Arc::new(PostgresDriver::new());
        let conn = driver.connect(link.as_str());
        if conn.is_err() {
            let info = "[rbatis] connect mysql server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(info);
        }
        return Result::Ok(conn.unwrap());
    } else {
        let info = "[rbatis] connect fail,not support database type:".to_string() + arg.db_type.as_str();
        error!("{}", info);
        return Result::Err(info);
    }
}
