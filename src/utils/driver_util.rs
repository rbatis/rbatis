use crate::core::db_config::DBConfig;
use mysql::Conn;

pub fn get_mysql_conn(arg: &DBConfig) -> Result<Conn, String> {
    let mut ops = mysql::OptsBuilder::new();
    ops.user(Some(arg.db_user.as_str()));
    ops.pass(Some(arg.db_pwd.as_str()));
    ops.db_name(Some(arg.db_name.as_str()));
    ops.ip_or_hostname(Some(arg.addr.as_str()));
    ops.tcp_port(arg.port as u16);
    let conn = Conn::new(ops);
    if conn.is_err() {
        return Result::Err("[rbatis] conn fail:".to_string() + conn.err().unwrap().to_string().as_str());
    }
    return Result::Ok(conn.unwrap());
}