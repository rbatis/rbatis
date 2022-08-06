pub mod sqlite_table_sync;

pub use sqlite_table_sync::*;


use std::any::Any;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use rbs::Value;
use crate::Error;
use rbdc::db::Connection;
use crate::executor::RBatisConnExecutor;

#[async_trait::async_trait]
pub trait TableSync {
    async fn sync(&self, rb: RBatisConnExecutor, table: Value,name:&str) -> Result<(), Error>;
}

pub struct RbatisTableSync {
    pub plugins: HashMap<String, Box<dyn TableSync>>,
}

impl Deref for RbatisTableSync {
    type Target = HashMap<String, Box<dyn TableSync>>;

    fn deref(&self) -> &Self::Target {
        &self.plugins
    }
}

impl DerefMut for RbatisTableSync {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.plugins
    }
}

impl RbatisTableSync {
    pub fn new() -> Self {
        Self {
            plugins: Default::default()
        }
    }
    pub async fn sync(&self, driver_type: &str, rb: RBatisConnExecutor, table: Value,name:&str) -> Result<(), Error> {
        let plugin = self.plugins.get(driver_type);
        match plugin {
            None => {
                Err(Error::from("not support or load plugin!"))
            }
            Some(plugin) => {
                plugin.sync(rb, table,name).await
            }
        }
    }
}
