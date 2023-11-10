use std::fmt::Formatter;
use std::time::Duration;
use async_trait::async_trait;
use futures_core::future::BoxFuture;
use rbs::Value;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::Error;
use rbdc::pool::conn_box::ConnectionBox;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;

#[derive(Debug)]
pub struct MobcPool {
    pub manager: ConnManagerProxy,
    pub inner: mobc::Pool<ConnManagerProxy>,
}
pub struct ConnManagerProxy {
    inner: ConnManager,
    conn: Option<mobc::Connection<ConnManagerProxy>>,
}

impl std::fmt::Debug for ConnManagerProxy{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<ConnManager> for ConnManagerProxy {
    fn from(value: ConnManager) -> Self {
        ConnManagerProxy {
            inner: value,
            conn: None,
        }
    }
}

#[async_trait]
impl Pool for MobcPool {
    fn new(manager: ConnManager) -> Result<Self, Error> where Self: Sized {
        Ok(Self {
            manager: manager.clone().into(),
            inner: mobc::Pool::new(manager.into()),
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v = self.inner.get().await.map_err(|e| Error::from(e.to_string()))?;
        let proxy = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(proxy))
    }

    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
        let v = self.inner.get_timeout(d).await.map_err(|e| Error::from(e.to_string()))?;
        let proxy = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(proxy))
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
        self.manager.inner.driver_type()
    }
}

#[async_trait]
impl mobc::Manager for ConnManagerProxy {
    type Connection = ConnectionBox;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.inner.connect().await
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        self.inner.check(conn).await
    }
}

impl Connection for ConnManagerProxy {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.conn.as_mut().expect("conn is none").get_rows(sql, params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.conn.as_mut().expect("conn is none").exec(sql, params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.conn.as_mut().expect("conn is none").ping()
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        self.conn.as_mut().expect("conn is none").close()
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {

    }
}