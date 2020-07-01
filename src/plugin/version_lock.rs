//TODO version lock

use rbatis_core::db::DriverType;
use serde_json::Value;
use rbatis_core::Error;

pub trait VersionLock: Send + Sync {

    fn make_update_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<String, rbatis_core::Error>;
}


pub struct RbatisLogicDeletePlugin{}


impl VersionLock for RbatisLogicDeletePlugin{


    fn make_update_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<Value>) -> Result<String, Error> {
        //TODO SET sql insert del_flag = 0
        unimplemented!();

        let mut new_sql= sql.to_string();
        new_sql = new_sql.replace("update ","UPDATE ");
        new_sql = new_sql.replace("set ","SET ");
        new_sql = new_sql.replace("where ","WHERE ");
    }
}