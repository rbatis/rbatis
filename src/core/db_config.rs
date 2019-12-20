use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, RbatisMacro)]
pub struct DBConfig {
    pub db_type: String,
    pub db_name:String,
    pub db_user: String,
    pub db_pwd: String,
    pub db_addr: String,
    pub db_port: i32,
}

impl DBConfig {
    pub fn new(link: String) -> Result<DBConfig, String> {
        if link.find("://").is_none() {
            return Result::Err("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ://".to_string());
        }
        let dbtype_cfg: Vec<&str> = link.split("://").collect();
        if dbtype_cfg[1].find("@").is_none() {
            return Result::Err("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss @".to_string());
        }
        let user_pwd_link: Vec<&str> = dbtype_cfg[1].split("@").collect();
        if user_pwd_link[0].find(":").is_none() {
            return Result::Err("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ':' of [user]:[password]".to_string());
        }
        let user_pwd: Vec<&str> = user_pwd_link[0].split(":").collect();

        if user_pwd_link[1].find("/").is_none() {
            return Result::Err("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss '/' of [port]/[db_name]".to_string());
        }
        let link_dbname: Vec<&str> = user_pwd_link[1].split("/").collect();

        if link_dbname[0].find(":").is_none() {
            return Result::Err("[rbatis] link must have [type]://[user]:[password]@[ip]:[port]/[db_name], miss ':' of [ip]:[port]".to_string());
        }
        let addr_port: Vec<&str> = link_dbname[0].split(":").collect();

        let db_type = dbtype_cfg[0];
        let db_user = user_pwd[0];
        let db_pwd = user_pwd[1];
        let addr = addr_port[0];
        let port = addr_port[1];
        let db_name=link_dbname[1];

        return Result::Ok(Self {
            db_type: db_type.to_string(),
            db_name: db_name.to_string(),
            db_user: db_user.to_string(),
            db_pwd: db_pwd.to_string(),
            db_addr: addr.to_string(),
            db_port: port.parse().unwrap(),
        });
    }

    pub fn to_string(&self) -> String {
        let templete = "#{db_type}://#{db_user}:#{db_pwd}@#{db_addr}:#{db_port}/#{db_name}";
        let link = templete.replace("#{db_type}", "postgres")
            .replace("#{db_user}", self.db_user.as_str())
            .replace("#{db_pwd}", self.db_pwd.as_str())
            .replace("#{db_addr}", self.db_addr.as_str())
            .replace("#{db_port}", self.db_port.to_string().as_str())
            .replace("#{db_name}", self.db_name.as_str());
        return link;
    }
}