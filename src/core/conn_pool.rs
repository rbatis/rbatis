use std::collections::HashMap;
use crate::core::db_config::DBConfig;
use crate::utils::driver_util;
use rdbc::Connection;

pub struct ConnPool {
    pub conn_map: HashMap<String, Box<dyn Connection>>,
}

impl ConnPool{
    pub fn new()->ConnPool{
        return Self{
            conn_map:HashMap::new(),
        }
    }

    pub fn get_conn(&mut self,id:String,conf:&DBConfig)->Result<Option<&mut Box<dyn Connection>>,String>{
        if self.conn_map.get(&id).is_some() {
            return Result::Ok(self.conn_map.get_mut(&id));
        }else{
            let mysql_coon = driver_util::get_conn(conf)?;
            self.conn_map.insert(id.clone(), mysql_coon);
            return Result::Ok(self.conn_map.get_mut(&id));
        }
    }

}

#[test]
pub fn test_pool() {
    let mut conn = ConnPool::new();
    let c=conn.conn_map.get_mut(&"".to_string());
    println!("{}",c.is_none());
}

