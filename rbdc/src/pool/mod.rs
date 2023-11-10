pub mod conn_manager;
pub mod conn_box;

use std::fmt::Debug;
use crate::db::Connection;
use crate::Error;
use crate::pool::conn_manager::ConnManager;
use std::time::Duration;
use async_trait::async_trait;
use rbs::Value;

#[async_trait]
pub trait Pool: Sync + Send + Debug {

    /// create an Pool,use ConnManager
    fn new(manager: ConnManager) -> Result<Self, Error> where Self: Sized;

    /// get an connection from pool
    async fn get(&self) -> Result<Box<dyn Connection>, Error>;

    /// get timeout from pool
    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error>;

    async fn set_conn_max_lifetime(&self, max_lifetime: Option<Duration>);

    async fn set_max_idle_conns(&self, n: u64);

    async fn set_max_open_conns(&self, n: u64);

    ///return state
    async fn state(&self) -> Value{ Value::Null  }

    fn driver_type(&self) -> &str;
}

#[cfg(test)]
mod test {}