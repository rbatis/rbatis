use std::error::Error;

use log::{error, info, warn};
use rdbc::{Connection, Driver};


use crate::db_config::DBConfig;
use rdbc::mysql::MySQLDriver;
use rdbc::postgres::PostgresDriver;
use crate::error::RbatisError;
use rdbc::sqlite::SqliteDriver;

/// fetch a database connection
pub fn get_conn_by_link(link: &str) -> Result<Box<dyn Connection>, RbatisError> {
    if link.is_empty()|| link.find(":").is_none(){
        return Err(RbatisError::from("[rbatis] error of driver link!".to_string()));
    }
    if link.starts_with("mysql") {
        let driver = MySQLDriver::new();
        let conn = driver.connect(link);
        if conn.is_err() {
            let info = "[rbatis] connect mysql server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(RbatisError::from(info));
        }
        return Result::Ok(conn.unwrap());
    } else if link.starts_with("postgres") {
        let driver: Box<dyn rdbc::Driver> = Box::new(PostgresDriver::new());
        let conn = driver.connect(link);
        if conn.is_err() {
            let info = "[rbatis] connect postgres server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(RbatisError::from(info));
        }
        return Result::Ok(conn.unwrap());
    } else if link.starts_with("sqlite") {
        let driver: Box<dyn rdbc::Driver> = Box::new(SqliteDriver::new());
        let conn = driver.connect(link);
        if conn.is_err() {
            let info = "[rbatis] connect sqlite server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(RbatisError::from(info));
        }
        return Result::Ok(conn.unwrap());
    } else {
        let sp:Vec<&str> =link.split(":").collect();
        let info = "[rbatis] connect fail,not support database type:".to_string() + sp[0];
        error!("{}", info);
        return Result::Err(RbatisError::from(info));
    }
}