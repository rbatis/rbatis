use rbatis::RBatis;
use rbatis::{dark_std::defer, Error};
use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
use std::str::FromStr;

/// Example: Custom connection pool using deadpool
///
/// This example demonstrates how to implement a custom connection pool using the third-party `deadpool` library.
/// For most use cases, the default connection pool is sufficient. See pool_config.rs for default pool configuration.
///
/// Usage: `rb.init_option::<SqliteDriver, SqliteConnectOptions, my_pool::DeadPool>(SqliteDriver {}, opts)`
#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    let rb = RBatis::new();
    let opts = SqliteConnectOptions::from_str("sqlite://target/sqlite.db")?;

    // Uncomment to use default pool instead:
    // let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, rbatis::DefaultPool>(SqliteDriver {}, opts);

    // Set custom pool implementation (using deadpool)
    let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, my_pool::DeadPool>(
        SqliteDriver {},
        opts,
    );
    //set pool max size
    let _ = rb.get_pool()?.set_max_open_conns(100).await;
    let _ = rb.get_pool()?.set_max_idle_conns(100).await;
    let _ = rb.get_pool()?.get().await;
    println!(">>>>> state={}", rb.get_pool()?.state().await);
    Ok(())
}

mod my_pool {
    use deadpool::managed::{Metrics, Object, RecycleError, RecycleResult, Timeouts};
    use deadpool::Status;
    use futures_core::future::BoxFuture;
    use futures_core::Stream;
    use rbatis::async_trait;
    use rbatis::rbdc::pool::{ConnectionGuard, ConnectionManager, Pool};
    use rbatis::rbdc::{db, Driver, Error};
    use rbatis::rbdc::{Connection, ExecResult, Row};
    use rbs::value::map::ValueMap;
    use rbs::{value, Value};
    use std::borrow::Cow;
    use std::fmt::{Debug, Formatter};
    use std::ops::Deref;
use std::pin::Pin;
    use std::sync::Arc;
use std::time::Duration;

    /// Connection wrapper that implements Connection trait by delegating to the pooled connection
    pub struct ConnWrapper {
        pub inner: Object<ConnManagerProxy>,
    }

    impl Debug for ConnWrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ConnWrapper").finish()
        }
    }

    impl db::Connection for ConnWrapper {
        fn exec_rows(
            &mut self,
            sql: &str,
            params: Vec<Value>,
        ) -> BoxFuture<
            '_,
            Result<Pin<Box<dyn Stream<Item = Result<Box<dyn Row>, Error>> + Send + '_>>, Error>,
        > {
            self.inner.exec_rows(sql, params)
        }

        fn exec(
            &mut self,
            sql: &str,
            params: Vec<Value>,
        ) -> BoxFuture<'_, Result<ExecResult, Error>> {
            self.inner.exec(sql, params)
        }

        fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            self.inner.ping()
        }

        fn close(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            self.inner.close()
        }
    }

    /// Manager for deadpool - creates and recycles ConnectionGuard
    pub struct ConnManagerProxy {
        inner: ConnectionManager,
    }

    impl ConnManagerProxy {
        pub fn new(manager: ConnectionManager) -> Self {
            Self { inner: manager }
        }
    }

    impl deadpool::managed::Manager for ConnManagerProxy {
        type Type = ConnectionGuard;

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
                return Err(RecycleError::Message(Cow::Owned(
                    "connection already closed".to_string(),
                )));
            }
            self.inner.check(obj).await?;
            Ok(())
        }
    }

    pub struct DeadPool {
        inner: deadpool::managed::Pool<ConnManagerProxy>,
        driver_type: String,
        driver: Arc<dyn Driver>,
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
        fn new(manager: ConnectionManager) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let driver = manager.driver();
            let driver_type = manager.driver_type().to_string();
            let manager_proxy = ConnManagerProxy::new(manager);
            let inner = deadpool::managed::Pool::builder(manager_proxy)
                .build()
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Self { inner, driver_type, driver})
        }

        async fn get(&self) -> Result<Box<dyn Connection>, Error> {
            let guard = self
                .inner
                .get()
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(ConnWrapper { inner: guard }))
        }

        async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
            let mut out = Timeouts::default();
            out.create = Some(d);
            out.wait = Some(d);
            let guard = self
                .inner
                .timeout_get(&out)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(ConnWrapper { inner: guard }))
        }

        async fn set_conn_max_lifetime(&self, _max_lifetime: Option<Duration>) {
            // unimplemented
        }

        async fn set_max_idle_conns(&self, _n: u64) {
            // unimplemented
        }

        async fn set_max_open_conns(&self, n: u64) {
            self.inner.resize(n as usize);
        }

        fn driver_type(&self) -> &str {
            &self.driver_type
        }

        async fn state(&self) -> Value {
            let mut m = ValueMap::new();
            let state = self.status();
            m.insert(value!("max_size"), value!(state.max_size));
            m.insert(value!("size"), value!(state.size));
            m.insert(value!("available"), value!(state.available));
            m.insert(value!("waiting"), value!(state.waiting));
            Value::Map(m)
        }

        fn driver(&self) -> &dyn Driver {
            self.driver.deref()
        }
    }
}
