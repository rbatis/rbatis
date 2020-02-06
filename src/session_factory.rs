use std::collections::HashMap;
use std::thread::ThreadId;
use std::time::Duration;

use rdbc::Connection;

use crate::abstract_session::AbstractSession;
use crate::error::RbatisError;
use crate::local_session::LocalSession;
use crate::utils::driver_util;

///链接工厂
pub trait SessionFactory {
    fn get_thread_session(&mut self, id: &String, driver: &str) -> Result<&mut LocalSession, RbatisError>;
    fn remove(&mut self, id: &String);
}


pub struct SessionFactoryCached {
    /// data 持有session所有权，当session被删除时，session即被销毁
    pub data: HashMap<String, LocalSession>,
    pub max_conn: usize,
}


unsafe impl Send for SessionFactoryCached {}

unsafe impl Sync for SessionFactoryCached {}

impl Drop for SessionFactoryCached {
    fn drop(&mut self) {
        self.data.clear();
    }
}

impl SessionFactory for SessionFactoryCached {
    fn get_thread_session(&mut self, id: &String, driver: &str) -> Result<&mut LocalSession, RbatisError> {
        self.gc();
        let item = self.data.get_mut(id);
        if item.is_some() {
            return Ok(self.data.get_mut(id).unwrap());
        } else {
            let session = LocalSession::new("", driver, None)?;
            self.data.insert(id.clone(), session);
            return Ok(self.data.get_mut(id).unwrap());
        }
    }

    fn remove(&mut self, id: &String) {
        self.data.remove(id);
    }
}

impl SessionFactoryCached {
    /// clean_no_tx_link:是否清理缓存无事务的链接，启用节省内存但是每个请求重复链接，不启用则复用链接性能高
    pub fn new(max_conn: usize) -> Self {
        return Self {
            data: HashMap::new(),
            max_conn,
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
            std::thread::sleep(Duration::from_millis(50));
            self.gc();
        }
    }
}