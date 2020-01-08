use std::error::Error;
use std::sync::Arc;

use log::{error, info, warn};
use mysql::Conn;
use postgres::Client;
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

pub fn get_mysql_conn(arg: &DBConfig) -> Result<Conn, String> {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some(arg.db_user.as_str()));
    ops.pass(Some(arg.db_pwd.as_str()));
    ops.db_name(Some(arg.db_name.as_str()));
    ops.ip_or_hostname(Some(arg.db_addr.as_str()));
    ops.tcp_port(arg.db_port as u16);
    let conn = Conn::new(ops);
    if conn.is_err() {
        let info = "[rbatis] connect mysql server fail:".to_string() + conn.err().unwrap().description();
        error!("{}", info);
        return Result::Err(info);
    }
    return Result::Ok(conn.unwrap());
}

pub fn get_postage_conn(arg: &DBConfig) -> Result<Client, String> {
    let link = arg.to_string();
    let clent_opt = Client::connect(link.as_str(), postgres::NoTls);
    if clent_opt.is_err() {
        let info = "[rbatis] connect postgres server fail:".to_string() + clent_opt.err().unwrap().description();
        error!("{}", info);
        return Result::Err(info);
    }
    return Result::Ok(clent_opt.unwrap());
}