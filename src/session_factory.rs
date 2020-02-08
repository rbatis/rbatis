use std::collections::HashMap;
use std::thread::ThreadId;
use std::time::Duration;

use rdbc::Connection;
use uuid::Uuid;

use crate::abstract_session::AbstractSession;
use crate::error::RbatisError;
use crate::local_session::LocalSession;
use crate::utils::driver_util;

///链接工厂
pub trait SessionFactory {
    fn get_thread_session(&mut self, id: &String, driver: &str) -> Result<&mut LocalSession, RbatisError>;
    fn remove(&mut self, id: &String);
    fn set_wait_type(&mut self, t: WaitType);
    fn wait_type(&self) -> WaitType;
}

#[derive(Clone, Copy, PartialEq)]
pub enum WaitType {
    Thread,
    Tokio,
}

///连接池session工厂  connection pool session factory
pub struct ConnPoolSessionFactory {
    /// data 持有session所有权，当session被删除时，session即被销毁
    pub data: HashMap<String, LocalSession>,
    pub max_conn: usize,
    pub max_wait_ms: u64,
    pub wait_type: WaitType,
}

impl Drop for ConnPoolSessionFactory {
    fn drop(&mut self) {
        self.data.clear();
    }
}

impl SessionFactory for ConnPoolSessionFactory {
    fn get_thread_session(&mut self, id: &String, driver: &str) -> Result<&mut LocalSession, RbatisError> {
        self.gc();
        let item = self.data.get_mut(id);
        if item.is_some() {
            return Ok(self.data.get_mut(id).unwrap());
        } else {
            let session = LocalSession::new(id, driver, None)?;
            self.data.insert(id.to_string(), session);
            return Ok(self.data.get_mut(id).unwrap());
        }
    }

    fn remove(&mut self, id: &String) {
        self.data.remove(id);
    }

    fn set_wait_type(&mut self, t: WaitType) {
        self.wait_type = t;
    }

    fn wait_type(&self) -> WaitType {
        self.wait_type
    }
}

impl ConnPoolSessionFactory {
    /// clean_no_tx_link:是否清理缓存无事务的链接，启用节省内存但是每个请求重复链接，不启用则复用链接性能高
    pub fn new(max_conn: usize, max_wait_ms: u64, wait_type: WaitType) -> Self {
        return Self {
            data: HashMap::new(),
            max_conn,
            max_wait_ms: 0,
            wait_type,
        };
    }
    ///清理所有不含事务的session
    pub fn gc(&mut self) {
        if self.data.len() <= self.max_conn {
            return;
        }
        let mut kvec = vec![];
        for (k, v) in &self.data {
            if !v.have_tx() {
                kvec.push(k.clone());
            }
        }
        //清理无用的链接
        for item in kvec {
            self.data.remove(&item);
        }
        if self.data.len() > self.max_conn {
            //持续GC直到小于最大连接数
            match self.wait_type {
                WaitType::Thread => {
                    std::thread::sleep(Duration::from_millis(self.max_wait_ms));
                }
                WaitType::Tokio => {
                    tokio::time::delay_for(Duration::from_millis(self.max_wait_ms));
                }
            }
            self.gc();
        }
    }
}