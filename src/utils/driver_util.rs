use crate::core::db_config::DBConfig;
use mysql::Conn;
use std::error::Error;
use postgres::Client;

pub fn get_mysql_conn(arg: &DBConfig) -> Result<Conn, String> {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some(arg.db_user.as_str()));
    ops.pass(Some(arg.db_pwd.as_str()));
    ops.db_name(Some(arg.db_name.as_str()));
    ops.ip_or_hostname(Some(arg.db_addr.as_str()));
    ops.tcp_port(arg.db_port as u16);
    let conn = Conn::new(ops);
    if conn.is_err() {
        return Result::Err("[rbatis] connect mysql server fail:".to_string() + conn.err().unwrap().description());
    }
    return Result::Ok(conn.unwrap());
}

pub fn get_postage_conn(arg: &DBConfig) -> Result<Client, String> {
    let link=arg.to_string();
    let clent_opt = Client::connect(link.as_str(), postgres::NoTls);
    if clent_opt.is_err() {
        return Result::Err("[rbatis] connect postgres server fail:".to_string() + clent_opt.err().unwrap().description());
    }
    return Result::Ok(clent_opt.unwrap());
}