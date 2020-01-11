use std::error::Error;
use std::sync::Arc;

use log::{error, info, warn};
use rdbc::{Connection, Driver};
use rdbc_mysql::MySQLDriver;
use rdbc_postgres::PostgresDriver;

use crate::db_config::DBConfig;


pub fn get_conn_by_link(link: &str) -> Result<Box<dyn Connection>, String> {
    if link.is_empty()|| link.find(":").is_none(){
        return Err("[rbatis] error of driver link!".to_string());
    }
    if link.starts_with("mysql") {
        let driver = Arc::new(MySQLDriver::new());
        let conn = driver.connect(link);
        if conn.is_err() {
            let info = "[rbatis] connect mysql server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(info);
        }
        return Result::Ok(conn.unwrap());
    } else if link.starts_with("postgres") {
        let driver: Arc<dyn rdbc::Driver> = Arc::new(PostgresDriver::new());
        let conn = driver.connect(link);
        if conn.is_err() {
            let info = "[rbatis] connect mysql server fail:".to_string() + format!("{:?}", conn.err().unwrap()).as_str();
            error!("{}", info);
            return Result::Err(info);
        }
        return Result::Ok(conn.unwrap());
    } else {
        let sp:Vec<&str> =link.split(":").collect();
        let info = "[rbatis] connect fail,not support database type:".to_string() + sp[0];
        error!("{}", info);
        return Result::Err(info);
    }
}

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
