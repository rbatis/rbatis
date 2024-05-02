use rbatis::dark_std::defer;
use rbatis::RBatis;
use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
use std::str::FromStr;

/// define a custom pool(my_pool::DeadPool) use `rb.init_option::<SqliteDriver, SqliteConnectOptions, my_pool::DeadPool>(SqliteDriver {}, opts)`
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    let rb = RBatis::new();
    let opts = SqliteConnectOptions::from_str("sqlite://target/sqlite.db").unwrap();
    //default_is//let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, rbatis::DefaultPool>(SqliteDriver{},opts);
    // set custom impl pool
    let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, my_pool::DeadPool>(
        SqliteDriver {},
        opts,
    );
    //set pool max size
    let _ = rb.get_pool().unwrap().set_max_open_conns(100).await;
    let _ = rb.get_pool().unwrap().set_max_idle_conns(100).await;
    let _ = rb.get_pool().unwrap().get().await;
    println!(">>>>> state={}", rb.get_pool().unwrap().state().await);
}

mod my_pool {
    use deadpool::managed::{Metrics, Object, RecycleError, RecycleResult, Timeouts};
    use deadpool::Status;
    use futures_core::future::BoxFuture;
    use rbatis::async_trait;
    use rbatis::rbdc::db::{Connection, ExecResult, Row};
    use rbatis::rbdc::pool::conn_box::ConnectionBox;
    use rbatis::rbdc::pool::conn_manager::ConnManager;
    use rbatis::rbdc::pool::Pool;
    use rbatis::rbdc::{db, Error};
    use rbs::value::map::ValueMap;
    use rbs::{to_value, Value};
    use std::borrow::Cow;
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
                return Err(RecycleError::Message(Cow::Owned("none".to_string())));
            }
            self.inner.check(obj).await?;
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
}
