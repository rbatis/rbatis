use std::collections::HashMap;
use std::thread::ThreadId;

use rdbc::Connection;

use crate::utils::driver_util;

///链接工厂
pub trait ConnFactory {
    fn get_thread_conn(&mut self, id: ThreadId, driver: &str) -> Result<&mut Box<dyn Connection>, String>;
}


pub struct ConnFactoryImpl {
    pub async_mode: bool,
    pub data: HashMap<ThreadId, Box<dyn Connection>>,
}


impl ConnFactory for ConnFactoryImpl {
    fn get_thread_conn(&mut self, id: ThreadId, driver: &str) -> Result<&mut Box<dyn Connection>, String> {
        println!("get_thread_driver:{:?},{}",id,driver);
        let item=self.data.get(&id);
        if item.is_some() {
            return Ok(self.data.get_mut(&id).unwrap());
        } else {
            let r = driver_util::get_conn_by_link(driver)?;
            self.data.insert(id.clone(), r);
            return Ok(self.data.get_mut(&id).unwrap());
        }
    }
}

impl ConnFactoryImpl {
    pub fn new(async_mode: bool) -> Self {
        return Self {
            async_mode,
            data: HashMap::new(),
        };
    }
}