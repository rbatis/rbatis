use std::time::Duration;
use async_trait::async_trait;
use futures_core::future::BoxFuture;
use rbs::Value;
use crate::db::{Connection, ExecResult, Row};
use crate::Error;
use crate::pool::{ConnectionBox, Pool};
use crate::pool::conn_manager::ConnManager;

#[derive(Debug)]
pub struct MobcPool {
    pub manager:ConnManager,
    pub inner: mobc::Pool<ConnManager>,
}

unsafe impl Sync for MobcPool {}
unsafe impl Send for MobcPool {}

#[async_trait]
impl Pool for MobcPool {
    fn new(manager: ConnManager) -> Result<Self,Error> where Self: Sized {
        Ok(Self {
            manager:manager.clone(),
            inner: mobc::Pool::new(manager)
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v = self.inner.get().await.map_err(|e|Error::from(e.to_string()))?;
        Ok(Box::new(v))
    }

    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
        let v = self.inner.get_timeout(d).await.map_err(|e|Error::from(e.to_string()))?;
        Ok(Box::new(v))
    }

    async fn set_conn_max_lifetime(&self, max_lifetime: Option<Duration>) {
       self.inner.set_conn_max_lifetime(max_lifetime).await;
    }

    async fn set_max_idle_conns(&self, n: u64) {
        self.inner.set_max_idle_conns(n).await;
    }

    async fn set_max_open_conns(&self, n: u64) {
        self.inner.set_max_open_conns(n).await;
    }

    fn driver_type(&self) -> &str {
        self.manager.driver_type()
    }

}

#[async_trait]
impl mobc::Manager for ConnManager {
    type Connection = ConnectionBox;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.connect().await
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        self.check( conn).await
    }
}


impl Connection for mobc::Connection<ConnManager>{
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.conn.as_mut().unwrap().get_rows(sql,params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.conn.as_mut().unwrap().exec(sql,params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.conn.as_mut().unwrap().ping()
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        self.conn.as_mut().unwrap().close()
    }
}