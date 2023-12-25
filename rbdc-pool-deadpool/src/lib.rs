use async_trait::async_trait;
use deadpool::managed::{Metrics, Object, RecycleError, RecycleResult, Timeouts};
use deadpool::Status;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::pool::conn_box::ConnectionBox;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;
use rbdc::{db, Error};
use rbs::value::map::ValueMap;
use rbs::{to_value, Value};
use std::fmt::{Debug, Formatter};
use std::time::Duration;

pub struct DeadPool {
    pub manager: ConnManagerProxy,
    pub inner: deadpool::managed::Pool<ConnManagerProxy>,
}

unsafe impl Send for DeadPool {}

unsafe impl Sync for DeadPool {}

impl Debug for DeadPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pool").finish()
    }
}

impl DeadPool {
    /// Retrieves Status of this Pool.
    pub fn status(&self) -> Status {
        self.inner.status()
    }
}

#[async_trait]
impl Pool for DeadPool {
    fn new(manager: ConnManager) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            manager: ConnManagerProxy {
                inner: manager.clone(),
                conn: None,
            },
            inner: deadpool::managed::Pool::builder(ConnManagerProxy {
                inner: manager,
                conn: None,
            })
            .build()
            .map_err(|e| Error::from(e.to_string()))?,
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v = self
            .inner
            .get()
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let conn = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(conn))
    }

    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
        let mut out = Timeouts::default();
        out.create = Some(d);
        out.wait = Some(d);
        let v = self
            .inner
            .timeout_get(&out)
            .await
            .map_err(|e| Error::from(e.to_string()))?;
        let conn = ConnManagerProxy {
            inner: v.manager_proxy.clone(),
            conn: Some(v),
        };
        Ok(Box::new(conn))
    }

    async fn set_conn_max_lifetime(&self, _max_lifetime: Option<Duration>) {
        //un impl
    }

    async fn set_max_idle_conns(&self, _n: u64) {
        //un impl
    }

    async fn set_max_open_conns(&self, n: u64) {
        self.inner.resize(n as usize)
    }

    fn driver_type(&self) -> &str {
        self.manager.inner.driver_type()
    }

    async fn state(&self) -> Value {
        let mut m = ValueMap::new();
        let state = self.status();
        m.insert(to_value!("max_size"), to_value!(state.max_size));
        m.insert(to_value!("size"), to_value!(state.size));
        m.insert(to_value!("available"), to_value!(state.available));
        m.insert(to_value!("waiting"), to_value!(state.waiting));
        Value::Map(m)
    }
}

pub struct ConnManagerProxy {
    pub inner: ConnManager,
    pub conn: Option<Object<ConnManagerProxy>>,
}

#[async_trait]
impl deadpool::managed::Manager for ConnManagerProxy {
    type Type = ConnectionBox;

    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.inner.connect().await
    }

    async fn recycle(
        &self,
        obj: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        if obj.conn.is_none() {
            return Err(RecycleError::StaticMessage("none"));
        }
        let mut copy: ConnectionBox = unsafe { std::mem::transmute_copy(&*obj) };
        copy.auto_close = false;
        self.inner.check(copy).await?;
        Ok(())
    }
}

impl db::Connection for ConnManagerProxy {
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
        Box::pin(async { self.conn.as_mut().unwrap().ping().await })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        if self.conn.is_none() {
            return Box::pin(async { Err(Error::from("conn is drop")) });
        }
        Box::pin(async { self.conn.as_mut().unwrap().close().await })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
