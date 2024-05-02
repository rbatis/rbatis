use crate::db::Connection;
use crate::pool::conn_manager::ConnManager;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub struct ConnectionBox {
    pub conn: Option<Box<dyn Connection>>,
    pub manager_proxy: ConnManager,
    pub auto_close: bool,
}
impl Debug for ConnectionBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionBox")
            .field("manager_proxy", &self.manager_proxy)
            .field("auto_close", &self.auto_close)
            .finish()
    }
}

unsafe impl Sync for ConnectionBox {}
impl Deref for ConnectionBox {
    type Target = Box<dyn Connection>;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl DerefMut for ConnectionBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for ConnectionBox {
    fn drop(&mut self) {
        if self.auto_close {
            if let Some(mut conn) = self.conn.take() {
                self.manager_proxy.spawn_task(async move {
                    let _ = conn.close().await;
                });
            }
        }
    }
}
