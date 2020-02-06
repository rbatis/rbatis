use std::collections::HashMap;
use std::thread::ThreadId;

use rdbc::Connection;

use crate::abstract_session::AbstractSession;
use crate::local_session::LocalSession;
use crate::utils::driver_util;
use crate::error::RbatisError;

///链接工厂
pub trait SessionFactory {
    fn get_thread_session(&mut self, id: &String, driver: &str) -> Result<&mut LocalSession, RbatisError>;
    fn remove(&mut self, id: &String);
}


pub struct SessionFactoryCached {
    /// data 持有session所有权，当session被删除时，session即被销毁
    pub data: HashMap<String, LocalSession>,
    ///是否清理缓存无事务的链接，启用节省内存，不启用则复用链接
    pub clean_no_tx_link: bool,
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
        if self.clean_no_tx_link {
            self.gc();
        }
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
    pub fn new(clean_no_tx_link: bool) -> Self {
        return Self {
            data: HashMap::new(),
            clean_no_tx_link: clean_no_tx_link,
        };
    }
    ///清理所有不含事务的session
    pub fn gc(&mut self){
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
    }
}