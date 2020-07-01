//TODO delete_flag plugin


use rbatis_core::db::DriverType;
use serde_json::Value;
use rbatis_core::Error;

pub trait LogicDelete: Send + Sync {

    fn make_delete_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<String, rbatis_core::Error>;
    fn make_select_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<serde_json::Value>) -> Result<String, rbatis_core::Error>;
}


pub struct RbatisLogicDeletePlugin{}


impl LogicDelete for RbatisLogicDeletePlugin{


    fn make_delete_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<Value>) -> Result<String, Error> {
        //TODO SET sql insert del_flag = 0
        unimplemented!();

        let mut new_sql= sql.to_string();
        new_sql = new_sql.replace("delete ","DELETE ");
    }

    fn make_select_sql(&self, driver_type: &DriverType, tx_id: &str, sql: &str, args: &Vec<Value>) -> Result<String, Error> {
        //TODO WHERE sql insert del_flag = 1
        unimplemented!();

        let mut new_sql= sql.to_string();
        new_sql = new_sql.replace("where ","WHERE ");
        new_sql = new_sql.replace("order by ","ORDER BY ");
    }
}