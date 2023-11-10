use crate::db::{Connection, ConnectOptions, Driver, ExecResult, Row};
use crate::{db, Error};
use async_trait::async_trait;
use deadpool::managed::{ Metrics, Object, PoolBuilder, PoolError, RecycleError, RecycleResult, Timeouts,};
use deadpool::Status;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use futures_core::future::BoxFuture;
use rbs::Value;
use crate::pool::conn_box::ConnectionBox;
use crate::pool::conn_manager::ConnManager;
use crate::pool::Pool;

/// deadpool
/// Max & idle connection lifetime see https://github.com/bikeshedder/deadpool/issues/178
#[derive(Clone)]
pub struct DeadPool {
    pub manager: ConnManager,
    pub inner: deadpool::managed::Pool<ConnManager>,
}

impl Debug for DeadPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pool")
            .field("manager", &self.manager)
            .finish()
    }
}


impl DeadPool {

    /// return driver name
    pub fn driver_type(&self) -> &str {
        self.manager.driver_type()
    }

    /// spawn task on runtime
    pub fn spawn_task<T>(&self, task: T)
        where
            T: Future + Send + 'static,
            T::Output: Send + 'static,
    {
        self.manager.spawn_task(task)
    }

    /**
     * Resize the pool. This change the `max_size` of the pool dropping
     * excess objects and/or making space for new ones.
     *
     * If the pool is closed this method does nothing. The [`DeadPool::status`] method
     * always reports a `max_size` of 0 for closed pools.
     */
    pub fn resize(&self, max_size: usize) {
        self.deref().resize(max_size);
    }

    /// Indicates whether this [`DeadPool`] has been closed.
    pub fn is_closed(&self) -> bool {
        self.deref().is_closed()
    }

    /// Closes this Pool.
    /// All current and future tasks waiting for Objects will return PoolError::Closed immediately.
    /// This operation resizes the pool to 0.
    pub fn close(&self) {
        self.deref().close();
    }

    /// Retrieves Status of this Pool.
    pub fn status(&self) -> Status {
        self.deref().status()
    }

    ///Get current timeout configuration
    pub fn timeouts(&self) -> Timeouts {
        self.deref().timeouts()
    }



    /// try get connection
    pub async fn try_get(&self) -> Result<Object<ConnManager>, PoolError<Error>> {
        let mut t = self.deref().timeouts();
        t.wait = Some(Duration::ZERO);
        self.deref().timeout_get(&t).await
    }

    pub fn new_url<Driver: crate::db::Driver + 'static>(
        d: Driver,
        url: &str,
    ) -> Result<Self, Error> {
        let manager = ConnManager::new(d, url)?;
        let p = DeadPool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        let pool = DeadPool {
            manager: manager,
            inner: p,
        };
        Ok(pool)
    }

    pub fn new<Driver: crate::db::Driver + 'static, ConnectOptions: crate::db::ConnectOptions>(
        d: Driver,
        o: ConnectOptions,
    ) -> Result<Self, Error> {
        let manager = ConnManager::new_opt(d, o);
        let inner = DeadPool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(DeadPool {
            manager: manager,
            inner: inner,
        })
    }

    pub fn new_box(d: Box<dyn Driver>, o: Box<dyn ConnectOptions>) -> Result<Self, Error> {
        let manager = ConnManager::new_opt_box(d, o);
        let inner = DeadPool::builder(manager.clone())
            .build()
            .map_err(|e| Error::from(e.to_string()))?;
        Ok(DeadPool {
            manager: manager,
            inner: inner,
        })
    }

    pub fn new_builder(
        builder: PoolBuilder<ConnManager, Object<ConnManager>>,
        d: Box<dyn Driver>,
        o: Box<dyn ConnectOptions>,
    ) -> Result<Self, Error> {
        let manager = ConnManager::new_opt_box(d, o);
        Ok(DeadPool {
            manager: manager,
            inner: builder.build().map_err(|e| Error::from(e.to_string()))?,
        })
    }

    pub fn builder(m: ConnManager) -> PoolBuilder<ConnManager, Object<ConnManager>> {
        deadpool::managed::Pool::builder(m)
    }
}

#[async_trait]
impl deadpool::managed::Manager for ConnManager{
    type Type = ConnectionBox;

    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.connect().await
    }

    async fn recycle(&self, obj: &mut Self::Type, _metrics: &Metrics) -> RecycleResult<Self::Error> {
        if obj.conn.is_none(){
            return Err(RecycleError::StaticMessage("none"));
        }
        let mut copy:ConnectionBox=unsafe{std::mem::transmute_copy(&obj)};
        copy.auto_close = false;
        self.check(copy).await?;
        Ok(())
    }
}

#[async_trait]
impl Pool for DeadPool{
    fn new(manager: ConnManager) -> Result<Self,Error> where Self: Sized {
        Ok(Self{
            manager:manager.clone(),
            inner: deadpool::managed::Pool::builder(manager).build().map_err(|e|Error::from(e.to_string()))?,
        })
    }

    async fn get(&self) -> Result<Box<dyn Connection>, Error> {
        let v=self.inner.get().await.map_err(|e|Error::from(e.to_string()))?;
        Ok(Box::new(v))
    }

    async fn get_timeout(&self, d: Duration) -> Result<Box<dyn Connection>, Error> {
        let mut out =Timeouts::default();
        out.create=Some(d);
        out.wait=Some(d);
        let v=self.inner.timeout_get(&out).await.map_err(|e|Error::from(e.to_string()))?;
        Ok(Box::new(v))
    }

    async fn set_conn_max_lifetime(&self, _max_lifetime: Option<Duration>) {
        //un impl
    }

    async fn set_max_idle_conns(&self, n: u64) {
        //un impl
    }

    async fn set_max_open_conns(&self, n: u64) {
        self.inner.resize(n as usize)
    }

    fn driver_type(&self) -> &str {
        self.manager.driver_type()
    }
}

impl db::Connection for Object<ConnManager>{
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.conn.as_mut().unwrap().get_rows(sql,params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.conn.as_mut().unwrap().exec(sql,params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async{
            self.conn.as_mut().unwrap().ping().await
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async{
            self.conn.as_mut().unwrap().close().await
        })
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::sync::Mutex;
    use async_trait::async_trait;
    use deadpool::managed::{Manager, Metrics, Object, RecycleError, RecycleResult};
    use once_cell::sync::Lazy;

    struct MockConn {
        pub id: i64,
    }

    pub static Drops: Lazy<Mutex<Vec<i64>>> = Lazy::new(|| {
        Mutex::new(vec![])
    });

    impl Drop for MockConn {
        fn drop(&mut self) {
            println!("mock drop={}", self.id);
            Drops.lock().unwrap().push(self.id);
        }
    }

    struct MockPool {}

    #[async_trait]
    impl Manager for MockPool {
        type Type = MockConn;
        type Error = ();

        async fn create(&self) -> Result<Self::Type, Self::Error> {
            println!("create");
            Ok(MockConn { id: fastdate::DateTime::now().unix_timestamp() })
        }

        async fn recycle(&self, obj: &mut Self::Type, metrics: &Metrics) -> RecycleResult<Self::Error> {
            return Err(RecycleError::Message(format!(
                "Connection is ping fail={}",
                ""
            )));
        }
    }

    #[tokio::test]
    async fn test_pool() {
        let p = deadpool::managed::Pool::builder(MockPool {}).build().unwrap();
        p.resize(5);
        let mut num = 0;
        for _ in 0..10 {
            let r: Object<MockPool> = p.get().await.unwrap();
            num += 1;
            drop(r);
        }
        println!("num={}", num);
        // drop(p);
        // sleep(Duration::from_secs(3));
        assert_eq!(Drops.lock().unwrap().len(), num);
    }


    #[derive(Debug)]
    pub struct FooError;

    pub struct FooConnection;

    impl FooConnection {
        pub async fn query(&self) -> String {
            "PONG".to_string()
        }
    }

    pub static Drops2: Lazy<Mutex<Vec<i64>>> = Lazy::new(|| {
        Mutex::new(vec![])
    });

    impl Drop for FooConnection {
        fn drop(&mut self) {
            println!("drop FooConnection");
            Drops2.lock().unwrap().push(0);
        }
    }


    pub struct FooManager;

    #[mobc::async_trait]
    impl mobc::Manager for FooManager {
        type Connection = FooConnection;
        type Error = String;

        async fn connect(&self) -> Result<Self::Connection, Self::Error> {
            Ok(FooConnection {})
        }

        async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
            Err("e".to_string())
        }
    }

    #[tokio::test]
    async fn test_pool_mobc() {
        Drops2.deref();
        let p = mobc::Pool::builder().max_idle(10).max_open(10).build(FooManager {});
        let mut num = 0;
        for _ in 0..10 {
            let mut r = p.get().await.expect("get fail");
            let r = &mut r as &mut FooConnection;
            num += 1;
            drop(r);
        }
        //println!("num={}",num);
        // drop(p);
        // sleep(Duration::from_secs(3));
        println!("n={}", Drops2.lock().unwrap().len());
        // assert_eq!(Drops.lock().unwrap().len(), num);
    }
}
