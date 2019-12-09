use std::collections::HashMap;
use crate::core::db_config::DBConfig;
use crate::utils::driver_util;
use mysql::Conn;

pub struct ConnPool {
    pub mysql_map: HashMap<String, mysql::Conn>,
    pub pg_map: HashMap<String, postgres::Client>,
}

impl ConnPool{
    pub fn new()->ConnPool{
        return Self{
            mysql_map: HashMap::new(),
            pg_map: HashMap::new(),
        }
    }
    pub fn get_mysql_conn(&mut self,name:String,conf:&DBConfig)->Result<Option<&mut Conn>,String>{
        if self.mysql_map.get(&name).is_some() {
            return Result::Ok(self.mysql_map.get_mut(&name));
        }else{
            let mysql_coon = driver_util::get_mysql_conn(conf)?;
            self.mysql_map.insert(name.clone(), mysql_coon);
            return Result::Ok(self.mysql_map.get_mut(&name));
        }
    }
    pub fn get_postage_conn(&mut self,name:String,conf:&DBConfig)->Result<Option<&mut postgres::Client>,String>{
        if self.pg_map.get(&name).is_some() {
            return Result::Ok(self.pg_map.get_mut(&name));
        }else{
            let mysql_coon = driver_util::get_postage_conn(conf)?;
            self.pg_map.insert(name.clone(), mysql_coon);
            return Result::Ok(self.pg_map.get_mut(&name));
        }
    }
}

#[test]
pub fn test_pool() {
    let mut conn = ConnPool::new();
    let c=conn.mysql_map.get_mut(&"".to_string());
    println!("{}",c.is_none());
    //  let v=conn.mysql_map.get_mut(&"s".to_string()).unwrap();
}

