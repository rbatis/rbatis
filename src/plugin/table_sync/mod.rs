pub mod sqlite_table_sync;

pub use sqlite_table_sync::*;


use std::any::Any;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use serde::Serialize;
use rbs::{to_value, Value};
use crate::Error;
use rbdc::db::Connection;
use crate::executor::RBatisConnExecutor;
use crate::utils::string_util::to_snake_name;

#[async_trait::async_trait]
pub trait TableSync {
    async fn sync(&self, rb: RBatisConnExecutor, table: Value, name: &str) -> Result<(), Error>;
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

impl Default for RbatisTableSync {
    fn default() -> Self {
        Self::new()
    }
}
impl RbatisTableSync {
    pub fn new() -> Self {
        Self {
            plugins: Default::default()
        }
    }
    pub async fn sync<Table: Serialize + Any>(&self, driver_type: &str, rb: RBatisConnExecutor, table: Table) -> Result<(), Error> {
        let plugin = self.plugins.get(driver_type);
        match plugin {
            None => {
                Err(Error::from("not support or load plugin!"))
            }
            Some(plugin) => {
                let name = std::any::type_name::<Table>();
                let struct_name = name.split("::").last().unwrap_or_default();
                let struct_name = to_snake_name(struct_name);
                log::info!("sync table_name:{}",struct_name);
                plugin.sync(rb, to_value!(table), &struct_name).await
            }
        }
    }
    pub async fn sync_with_table_name<Table: Serialize + Any>(&self, driver_type: &str, rb: RBatisConnExecutor, table: Table, table_name: &str) -> Result<(), Error> {
        let plugin = self.plugins.get(driver_type);
        match plugin {
            None => {
                Err(Error::from("not support or load plugin!"))
            }
            Some(plugin) => {
                log::info!("sync table_name:{}",table_name);
                plugin.sync(rb, to_value!(table), table_name).await
            }
        }
    }
}
