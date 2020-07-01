//TODO version lock

use rbatis_core::db::DriverType;
use serde_json::Value;
use rbatis_core::Error;

pub trait VersionLock<'a>: Send + Sync {
    fn column(&'a self) -> &'a str;
    fn make_update_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>, arg: &serde_json::Value) -> Result<(String,Vec<Value>), rbatis_core::Error>;
}


pub struct RbatisLogicDeletePlugin {
    pub column: String,
}


impl<'a> VersionLock<'a> for RbatisLogicDeletePlugin {
    fn column(&'a self) -> &'a str {
        self.column.as_str()
    }


    fn make_update_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<Value>, arg: &serde_json::Value) -> Result<(String,Vec<Value>), Error> {
        //TODO SET sql insert del_flag = 0

        unimplemented!();
        let mut new_sql = sql.to_string();
        new_sql = new_sql.replace("set ", "SET ");
        new_sql = new_sql.replace("where ", "WHERE ");

        //do replace
        let mut sql_args=args.clone();

        new_sql = new_sql.replace("SET ", format!("SET {} = {} + 1, ", self.column.as_str(), self.column.as_str()).as_str());
        sql_args.insert(0,serde_json::Value::String(self.column().to_string()));
        sql_args.insert(0,serde_json::Value::String(self.column().to_string()));
        let version = arg.get(self.column()).unwrap_or(&serde_json::Value::Null);
        if version.is_null() {
            return Err(Error::from(format!("[rbatis] version_lock plugin arg must have arg: {}", self.column())));
        }
        let version_num = version.as_u64().unwrap_or(0);
        new_sql = new_sql.replace("WHERE ", format!("WHERE {} = {}", self.column.as_str(), version_num).as_str());


        // return Ok(new_sql,);
        return Err(Error::from(format!("[rbatis] version_lock plugin arg must have arg: {}", self.column())));
    }
}

mod Test{
    use crate::plugin::version_lock::{RbatisLogicDeletePlugin, VersionLock};
    use rbatis_core::db::DriverType;

    #[test]
    pub fn test_make_update_sql(){
          let plugin = RbatisLogicDeletePlugin{ column: "del_flag".to_string() };
           plugin.make_update_sql(&DriverType::Mysql,"",sql,)
    }
}