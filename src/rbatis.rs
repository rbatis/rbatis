use crate::executor::{Executor, RBatisConnExecutor, RBatisTxExecutor};
use crate::intercept_log::LogInterceptor;
use crate::plugin::intercept::Intercept;
use crate::snowflake::{Snowflake};
use crate::table_sync::{sync, ColumnMapper};
use crate::{DefaultPool, Error};
use dark_std::sync::SyncVec;
use log::LevelFilter;
use rbdc::pool::conn_manager::ConnManager;
use rbdc::pool::Pool;
use rbs::to_value;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

/// RBatis engine
#[derive(Clone, Debug)]
pub struct RBatis {
    // the connection pool
    pub pool: Arc<OnceLock<Box<dyn Pool>>>,
    // intercept vec(default the intercepts[0] is a log interceptor)
    pub intercepts: Arc<SyncVec<Arc<dyn Intercept>>>,
    //rb task id gen
    pub task_id_generator: Arc<Snowflake>,
}

impl Default for RBatis {
    fn default() -> RBatis {
        RBatis {
            pool: Arc::new(Default::default()),
            intercepts: Arc::new(SyncVec::new()),
            task_id_generator: Arc::new(Snowflake::default()),
        }
    }
}

impl RBatis {
    /// create an RBatis
    /// add intercept use LogInterceptor
    pub fn new() -> Self {
        let rb = RBatis::default();
        //default use LogInterceptor
        rb.intercepts
            .push(Arc::new(LogInterceptor::new(LevelFilter::Info)));
        rb
    }

    /// self.init(driver, url)? and self.try_acquire().await? a connection.
    /// DefaultPool use FastPool
    pub async fn link<Driver: rbdc::db::Driver + 'static>(
        &self,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        self.init(driver, url)?;
        self.try_acquire().await?;
        Ok(())
    }

    /// init pool.
    /// The default connection pool only binds one type of database driver, please use separate RBatis for different database drivers
    /// DefaultPool is FastPool,if you want other pool please use init_option
    pub fn init<Driver: rbdc::db::Driver + 'static>(
        &self,
        driver: Driver,
        url: &str,
    ) -> Result<(), Error> {
        if url.is_empty() {
            return Err(Error::from("[rb] link url is empty!"));
        }
        let mut option = driver.default_option();
        option.set_uri(url)?;
        let pool = DefaultPool::new(ConnManager::new_arc(Arc::new(Box::new(driver)), Arc::new(option)))?;
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        Ok(())
    }

    /// init pool by DBPoolOptions and Pool
    /// for example:
    ///```
    /// use rbatis::{DefaultPool, RBatis};
    /// use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
    /// let rb=RBatis::new();
    ///
    /// let opts=SqliteConnectOptions::new();
    /// let _ = rb.init_option::<SqliteDriver, SqliteConnectOptions, DefaultPool>(SqliteDriver{},opts);
    /// ```
    ///
    pub fn init_option<
        Driver: rbdc::db::Driver + 'static,
        ConnectOptions: rbdc::db::ConnectOptions,
        Pool: rbdc::pool::Pool + 'static,
    >(
        &self,
        driver: Driver,
        option: ConnectOptions,
    ) -> Result<(), Error> {
        let pool = Pool::new(ConnManager::new_arc(Arc::new(Box::new(driver)), Arc::new(Box::new(option))))?;
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        Ok(())
    }

    pub fn init_pool<Pool: rbdc::pool::Pool + 'static>(&self, pool: Pool) -> Result<(), Error> {
        self.pool
            .set(Box::new(pool))
            .map_err(|_e| Error::from("pool set fail!"))?;
        Ok(())
    }

    #[deprecated(note = "please use init_option()")]
    pub fn init_opt<
        Driver: rbdc::db::Driver + 'static,
        ConnectOptions: rbdc::db::ConnectOptions,
    >(
        &self,
        driver: Driver,
        options: ConnectOptions,
    ) -> Result<(), Error> {
        self.init_option::<Driver, ConnectOptions, DefaultPool>(driver, options)
    }

    /// set_intercepts for many
    pub fn set_intercepts(&mut self, arg: Vec<Arc<dyn Intercept>>) {
        self.intercepts = Arc::new(SyncVec::from(arg));
    }

    /// get conn pool
    ///
    /// can set option for example:
    /// ```rust
    /// use rbatis::RBatis;
    /// #[tokio::main]
    /// async fn main(){
    ///   let rb = RBatis::new();
    ///   rb.init(rbdc_sqlite::driver::SqliteDriver{},"sqlite://target/sqlite.db").unwrap();
    ///   rb.get_pool().unwrap().set_max_open_conns(10).await;
    /// }
    /// ```
    pub fn get_pool(&self) -> Result<&dyn Pool, Error> {
        let p = self
            .pool
            .get()
            .ok_or_else(|| Error::from("[rb] rbatis pool not inited!"))?;
        Ok(p.deref())
    }

    /// get driver type
    pub fn driver_type(&self) -> Result<&str, Error> {
        let pool = self.get_pool()?;
        Ok(pool.driver_type())
    }

    /// get an DataBase Connection used for the next step
    pub async fn acquire(&self) -> Result<RBatisConnExecutor, Error> {
        let pool = self.get_pool()?;
        let conn = pool.get().await?;
        Ok(RBatisConnExecutor::new(self.task_id_generator.generate(), conn, self.clone()))
    }

    /// try get an DataBase Connection used for the next step
    pub async fn try_acquire(&self) -> Result<RBatisConnExecutor, Error> {
        self.try_acquire_timeout(Duration::from_secs(0)).await
    }

    /// try get an DataBase Connection used for the next step
    pub async fn try_acquire_timeout(&self, d: Duration) -> Result<RBatisConnExecutor, Error> {
        let pool = self.get_pool()?;
        let conn = pool.get_timeout(d).await?;
        Ok(RBatisConnExecutor::new(self.task_id_generator.generate(), conn, self.clone()))
    }

    /// get an DataBase Connection,and call begin method,used for the next step
    pub async fn acquire_begin(&self) -> Result<RBatisTxExecutor, Error> {
        let conn = self.acquire().await?;
        Ok(conn.begin().await?)
    }

    /// try get an DataBase Connection,and call begin method,used for the next step
    pub async fn try_acquire_begin(&self) -> Result<RBatisTxExecutor, Error> {
        let conn = self.try_acquire().await?;
        let executor = conn.begin().await?;
        Ok(executor)
    }

    /// is debug mode
    pub fn is_debug_mode(&self) -> bool {
        crate::decode::is_debug_mode()
    }

    /// get intercept from name
    /// the default name just like `let name = std::any::type_name::<LogInterceptor>()`
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let name = std::any::type_name::<MockIntercept>();
    ///  let intercept = rb.get_intercept_dyn(name);
    /// ```
    pub fn get_intercept_dyn(&self, name: &str) -> Option<&dyn Intercept> {
        for x in self.intercepts.iter() {
            if name == x.name() {
                return Some(x.as_ref());
            }
        }
        None
    }

    /// get intercept from name
    ///  ```rust
    /// use std::sync::Arc;
    /// use async_trait::async_trait;
    /// use rbatis::RBatis;
    /// use rbatis::intercept::{Intercept};
    ///
    /// #[derive(Debug)]
    /// pub struct MockIntercept {
    /// }
    /// #[async_trait]
    /// impl Intercept for MockIntercept {
    /// }
    ///  //use get_intercept_type
    ///  let mut rb = RBatis::new();
    ///  rb.set_intercepts(vec![Arc::new(MockIntercept{})]);
    ///  let intercept = rb.get_intercept::<MockIntercept>();
    /// ```
    pub fn get_intercept<T: Intercept>(&self) -> Option<&T> {
        let name = std::any::type_name::<T>();
        for item in self.intercepts.iter() {
            if name == item.name() {
                //this is safe
                let call: &T = unsafe { std::mem::transmute_copy(&item.as_ref()) };
                return Some(call);
            }
        }
        None
    }

    /// create table if not exists, add column if not exists
    ///
    /// ```rust
    /// use rbatis::executor::Executor;
    /// use rbatis::RBatis;
    /// use rbatis::table_sync::{SqliteTableMapper};
    ///
    /// /// let rb = RBatis::new();
    /// /// let conn = rb.acquire().await;
    /// pub async fn do_sync_table(conn: &dyn Executor){
    ///       let map = rbs::to_value!{
    ///             "id":"INT",
    ///             "name":"TEXT",
    ///      };
    ///      let _ = RBatis::sync(conn,&SqliteTableMapper{},&map,"user").await;
    /// }
    /// ```
    ///
    /// sync table struct
    /// ```rust
    /// use rbatis::executor::Executor;
    /// use rbatis::RBatis;
    /// use rbatis::table_sync::{SqliteTableMapper};
    ///
    /// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    /// pub struct User{
    ///   pub id:String,
    ///   pub name: Option<String>
    /// }
    ///
    /// /// let rb = RBatis::new();
    /// /// let conn = rb.acquire().await;
    /// pub async fn do_sync_table(conn: &dyn Executor){
    ///      let table = User{id: "".to_string(), name: Some("".to_string())};
    ///      let _ = RBatis::sync(conn,&SqliteTableMapper{},&table,"user").await;
    /// }
    /// ```
    ///
    /// sync table struct (custom string column type)
    /// ```rust
    /// use rbatis::executor::Executor;
    /// use rbatis::RBatis;
    /// use rbatis::table_sync::{MysqlTableMapper};
    ///
    /// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    /// pub struct User{
    ///   pub id:String,
    ///   pub name: Option<String>
    /// }
    ///
    /// /// let rb = RBatis::new();
    /// /// let conn = rb.acquire().await;
    /// pub async fn do_sync_table_mysql(conn: &dyn Executor){
    ///      //empty string: auto create type,  "VARCHAR(50)" -> sqlite type
    ///      let table = User{id: "".to_string(), name: Some("VARCHAR(50)".to_string())};
    ///      let _ = RBatis::sync(conn,&MysqlTableMapper{},&table,"user").await;
    /// }
    /// ```
    pub async fn sync<T: Serialize>(
        executor: &dyn Executor,
        column_mapper: &dyn ColumnMapper,
        table: &T,
        table_name: &str,
    ) -> Result<(), Error> {
        sync(executor, column_mapper, to_value!(table), table_name).await
    }
}
