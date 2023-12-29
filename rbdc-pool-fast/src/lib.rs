use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::pool::conn_box::ConnectionBox;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;
use rbdc::Error;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::time::Duration;

#[derive(Debug)]
pub struct FastPool {
    pub manager: ConnManagerProxy,
    pub inner: fast_pool::Pool<ConnManagerProxy>,
}

#[derive(Debug)]
pub struct ConnManagerProxy {
    inner: ConnManager,
    conn: Option<fast_pool::ConnectionBox<ConnManagerProxy>>,
}

impl From<ConnManager> for ConnManagerProxy {
    fn from(value: ConnManager) -> Self {
        ConnManagerProxy {
            inner: value,
            conn: None,
        }
    }
}

#[async_trait::async_trait]
impl Pool for FastPool {
    fn new(manager: ConnManager) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            manager: manager.clone().into(),
            inner: fast_pool::Pool::new(manager.into()),
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v = self
            .inner
            .get()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let proxy = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(proxy))
    }

    async fn get_timeout(&self, mut d: Duration) -> Result<Box<dyn Connection>, Error> {
        if d.is_zero() {
            let state = self.inner.state();
            if state.in_use < state.max_open {
                d = Duration::from_secs(10);
            } else {
                return Err(Error::from("Time out in the connection pool"));
            }
        }
        let v = self
            .inner
            .get_timeout(Some(d))
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let proxy = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(proxy))
    }

    async fn set_conn_max_lifetime(&self, _max_lifetime: Option<Duration>) {}

    async fn set_max_idle_conns(&self, _n: u64) {}

    async fn set_max_open_conns(&self, n: u64) {
        self.inner.set_max_open(n);
    }

    fn driver_type(&self) -> &str {
        self.manager.inner.driver_type()
    }

    async fn state(&self) -> Value {
        let mut m = ValueMap::with_capacity(10);
        let state = self.inner.state();
        m.insert("max_open".to_string().into(), state.max_open.into());
        m.insert("connections".to_string().into(), state.connections.into());
        m.insert("in_use".to_string().into(), state.in_use.into());
        m.insert("idle".to_string().into(), state.idle.into());
        m.insert("waits".to_string().into(), state.waits.into());
        // m.insert(
        //     "wait_duration".to_string().into(),
        //     to_value!(state.wait_duration),
        // );
        // m.insert(
        //     "max_idle_closed".to_string().into(),
        //     state.max_idle_closed.into(),
        // );
        // m.insert(
        //     "max_lifetime_closed".to_string().into(),
        //     state.max_lifetime_closed.into(),
        // );
        Value::Map(m)
    }
}

impl fast_pool::Manager for ConnManagerProxy {
    type Connection = ConnectionBox;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.inner.connect().await
    }

    async fn check(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let r = self.inner.check(conn).await;
        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl Connection for ConnManagerProxy {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().get_rows(sql, params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().exec(sql, params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().ping()
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        self.conn.as_mut().unwrap().close()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
