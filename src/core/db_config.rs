use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone,RbatisMacro)]
pub struct DBConfig{
    db_type:String,
    db_user:String,
    db_pwd:String,
    addr:String,
    port:i32,
}

impl DBConfig{

    pub fn new(link:String)->Self{
        if link.find("://").is_none(){
            panic!("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ://");
        }
        let dbtype_cfg: Vec<&str> = link.split("://").collect();
        if dbtype_cfg[1].find("@").is_none(){
            panic!("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss @");
        }
        let user_pwd_link: Vec<&str> = dbtype_cfg[1].split("@").collect();
        if user_pwd_link[0].find(":").is_none(){
            panic!("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ':' of [user]:[password]");
        }
        let user_pwd: Vec<&str> = user_pwd_link[0].split(":").collect();

        if user_pwd_link[1].find("/").is_none(){
            panic!("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss '/' of [port]/[db_name]");
        }
        let link_dbname: Vec<&str> = user_pwd_link[1].split("/").collect();

        if link_dbname[0].find(":").is_none(){
            panic!("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ':' of [ip]:[port]");
        }
        let addr_port: Vec<&str> = link_dbname[0].split(":").collect();

        let db_type=dbtype_cfg[0];
        let db_user=user_pwd[0];
        let db_pwd=user_pwd[1];
        let addr=addr_port[0];
        let port=addr_port[1];

        return Self{
            db_type: db_type.to_string(),
            db_user: db_user.to_string(),
            db_pwd: db_pwd.to_string(),
            addr: addr.to_string(),
            port: port.parse().unwrap(),
        }
    }
}