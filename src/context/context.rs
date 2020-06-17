use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use rbatis_core::mysql::MySqlConnection;
use rbatis_core::pool::PoolConnection;

//TODO impl Context hold tx
pub struct Context {
    //TODO Arc use async std's Arc ???
    tx_map: Mutex<RefCell<HashMap<String, PoolConnection<MySqlConnection>>>>,
}


impl Context {
    pub fn new() -> Context {
        Context {
            tx_map: Mutex::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn put(&self, key: &str, value: PoolConnection<MySqlConnection>) {
        let mut m = self.tx_map.lock().unwrap();
        let map = m.get_mut();
        map.insert(key.to_string(), value);
    }
}