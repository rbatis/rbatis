use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use bson2::Bson;
use bson2::spec::BinarySubtype;
use futures::Future;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde::{Serialize, Serializer};

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::crud::{CRUD, CRUDMut};
use crate::DriverType;
use crate::plugin::page::{IPageRequest, Page};
use crate::rbatis::Rbatis;
use crate::utils::string_util;
use futures::executor::block_on;
use rbatis_core::{DateTimeNative, Format};
use crate::snowflake::new_snowflake_id;


/// must be only one have Some(Value)
/// (&rb).into()
/// &mut tx.as_executor()
/// &mut conn.as_executor()
/// &mut guard.as_executor()
/// (&mut tx).into()
/// (&mut conn).into()
/// (&mut guard).into()
#[derive(Debug)]
pub enum RbatisExecutor<'r, 'inner> where 'inner: 'r {
    RB(&'r Rbatis),
    Conn(&'r mut RBatisConnExecutor<'inner>),
    TX(&'r mut RBatisTxExecutor<'inner>),
    TxGuard(&'r mut RBatisTxExecutorGuard<'inner>),
}


impl RbatisExecutor<'_, '_> {
    pub async fn fetch_page<T>(&mut self, sql: &str, args: Vec<Bson>, page_request: &dyn IPageRequest) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync {
        match self {
            RbatisExecutor::RB(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            RbatisExecutor::Conn(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            RbatisExecutor::TX(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
            RbatisExecutor::TxGuard(rb) => {
                return rb.fetch_page(sql, args, page_request).await;
            }
        }
    }

    pub async fn exec(&mut self, sql: &str, args: Vec<Bson>) -> Result<DBExecResult, Error> {
        match self {
            RbatisExecutor::RB(rb) => {
                return rb.exec(sql, args).await;
            }
            RbatisExecutor::Conn(rb) => {
                return rb.exec(sql, args).await;
            }
            RbatisExecutor::TX(rb) => {
                return rb.exec(sql, args).await;
            }
            RbatisExecutor::TxGuard(rb) => {
                return rb.exec(sql, args).await;
            }
        }
    }

    pub async fn fetch<T>(&mut self, sql: &str, args: Vec<Bson>) -> Result<T, Error> where T: DeserializeOwned {
        match self {
            RbatisExecutor::RB(rb) => {
                return rb.fetch(sql, args).await;
            }
            RbatisExecutor::Conn(rb) => {
                return rb.fetch(sql, args).await;
            }
            RbatisExecutor::TX(rb) => {
                return rb.fetch(sql, args).await;
            }
            RbatisExecutor::TxGuard(rb) => {
                return rb.fetch(sql, args).await;
            }
        }
    }
}

impl<'r, 'inner> RbatisRef for RbatisExecutor<'r, 'inner> {
    fn get_rbatis(&self) -> &Rbatis {
        match self {
            RbatisExecutor::RB(rb) => {
                rb
            }
            RbatisExecutor::Conn(rb) => {
                rb.get_rbatis()
            }
            RbatisExecutor::TX(rb) => {
                rb.get_rbatis()
            }
            RbatisExecutor::TxGuard(rb) => {
                rb.get_rbatis()
            }
        }
    }
}

impl<'r, 'inner> From<&'r Rbatis> for RbatisExecutor<'r, 'inner> {
    fn from(arg: &'r Rbatis) -> Self {
        Self::RB(arg)
    }
}

impl<'r, 'inner> From<&'r mut RBatisConnExecutor<'inner>> for RbatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut RBatisConnExecutor<'inner>) -> Self {
        Self::Conn(arg)
    }
}

impl<'r, 'inner> From<&'r mut RBatisTxExecutor<'inner>> for RbatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut RBatisTxExecutor<'inner>) -> Self {
        Self::TX(arg)
    }
}

impl<'r, 'inner> From<&'r mut RBatisTxExecutorGuard<'inner>> for RbatisExecutor<'r, 'inner> {
    fn from(arg: &'r mut RBatisTxExecutorGuard<'inner>) -> Self {
        Self::TxGuard(arg)
    }
}

#[async_trait]
pub trait RbatisRef {
    fn get_rbatis(&self) -> &Rbatis;

    fn driver_type(&self) -> crate::Result<DriverType> {
        self.get_rbatis().driver_type()
    }

    /// bind arg into DBQuery
    fn bind_arg<'arg>(
        &self,
        driver_type: &DriverType,
        sql: &'arg str,
        arg: Vec<Bson>,
    ) -> Result<DBQuery<'arg>, Error> {
        let mut q: DBQuery = driver_type.make_db_query(sql)?;
        for x in arg {
            (self.get_rbatis().encoder)(&mut q, x)?;
        }
        return Ok(q);
    }
}

impl RbatisRef for Rbatis {
    fn get_rbatis(&self) -> &Rbatis {
        &self
    }
}

#[async_trait]
pub trait ExecutorMut: RbatisRef {
    async fn exec(&mut self, sql: &str, args: Vec<bson2::Bson>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&mut self, sql: &str, args: Vec<bson2::Bson>) -> Result<T, Error> where T: DeserializeOwned;
}

#[derive(Debug)]
pub struct RBatisConnExecutor<'a> {
    pub conn: DBPoolConn<'a>,
    pub rb: &'a Rbatis,
}

impl<'b> RBatisConnExecutor<'b> {
    pub fn as_executor<'a>(&'a mut self) -> RbatisExecutor<'a, 'b> {
        RbatisExecutor::Conn(self)
    }
}

// bson vec to string
fn bson_arr_to_string(arg: Vec<Bson>) -> (Vec<Bson>, String) {
    let b = Bson::Array(arg);
    #[cfg(feature = "format_bson")]
        {
            let s = b.do_format();
            log::info!("[rbatis] [format_bson] => {}", s);
        }

    let s = b.to_string();
    return match b {
        Bson::Array(arr) => {
            (arr, s)
        }
        _ => {
            (vec![], s)
        }
    };
}

#[async_trait]
impl<'a> ExecutorMut for RBatisConnExecutor<'_> {
    async fn exec(&mut self, sql: &str, mut args: Vec<bson2::Bson>) -> Result<DBExecResult, Error> {
        let rb_task_id = new_snowflake_id();
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_rbatis().log_plugin.info(rb_task_id,
                                              &format!(
                                                  "Exec   ==> {}\n{}[rbatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        let result;
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type(), &sql, args)?;
            result = self.conn.exec_prepare(q).await;
        } else {
            result = self.conn.exec(&sql).await;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.get_rbatis().log_plugin.info(rb_task_id,
                                                      &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.get_rbatis().log_plugin
                        .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    async fn fetch<T>(&mut self, sql: &str, mut args: Vec<bson2::Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let rb_task_id = new_snowflake_id();
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_rbatis().log_plugin.info(rb_task_id,
                                              &format!(
                                                  "Fetch  ==> {}\n{}[rbatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type(), &sql, args)?;
            let result = self.conn.fetch_parperd(q).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(rb_task_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(rb_task_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(rb_task_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        }
    }
}

impl RbatisRef for RBatisConnExecutor<'_> {
    fn get_rbatis(&self) -> &Rbatis {
        self.rb
    }
}

impl<'a> RBatisConnExecutor<'a> {
    pub async fn begin(self) -> crate::Result<RBatisTxExecutor<'a>> {
        let tx = self.conn.begin().await?;
        return Ok(RBatisTxExecutor {
            tx_id: new_snowflake_id(),
            conn: tx,
            rb: self.rb,
        });
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor<'a> {
    pub tx_id: i64,
    pub conn: DBTx<'a>,
    pub rb: &'a Rbatis,
}

impl<'a, 'b> RBatisTxExecutor<'b> {
    pub fn as_executor(&'a mut self) -> RbatisExecutor<'a, 'b> {
        self.into()
    }
}


#[async_trait]
impl<'a> ExecutorMut for RBatisTxExecutor<'_> {
    async fn exec(&mut self, sql: &str, mut args: Vec<bson2::Bson>) -> Result<DBExecResult, Error> {
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_rbatis().log_plugin.info(self.tx_id,
                                              &format!(
                                                  "Exec   ==> {}\n{}[rbatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        let result;
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, args)?;
            result = self.conn.exec_prepare(q).await;
        } else {
            result = self.conn.exec(&sql).await;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            match &result {
                Ok(result) => {
                    self.get_rbatis().log_plugin.info(self.tx_id,
                                                      &format!("RowsAffected <== {}", result.rows_affected),
                    );
                }
                Err(e) => {
                    self.get_rbatis().log_plugin
                        .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                }
            }
        }
        return result;
    }

    async fn fetch<T>(&mut self, sql: &str, mut args: Vec<bson2::Bson>) -> Result<T, Error> where T: DeserializeOwned {
        let mut sql = sql.to_string();
        let is_prepared = args.len() > 0;
        for item in &self.get_rbatis().sql_intercepts {
            item.do_intercept(self.get_rbatis(), &mut sql, &mut args, is_prepared)?;
        }
        if self.get_rbatis().log_plugin.is_enable() {
            let (_args, args_string) = bson_arr_to_string(args);
            args = _args;
            self.get_rbatis().log_plugin.info(self.tx_id,
                                              &format!(
                                                  "Fetch  ==> {}\n{}[rbatis]                      Args   ==> {}",
                                                  &sql,
                                                  string_util::LOG_SPACE,
                                                  args_string
                                              ),
            );
        }
        if is_prepared {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &sql, args)?;
            let result = self.conn.fetch_parperd(q).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(self.tx_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        } else {
            let result = self.conn.fetch(&sql.to_owned()).await;
            if self.get_rbatis().log_plugin.is_enable() {
                match &result {
                    Ok(result) => {
                        self.get_rbatis().log_plugin
                            .info(self.tx_id, &format!("ReturnRows <== {}", result.1));
                    }
                    Err(e) => {
                        self.get_rbatis().log_plugin
                            .error(self.tx_id, &format!("ReturnErr  <== {}", e));
                    }
                }
            }
            return Ok(result?.0);
        }
    }
}

impl RbatisRef for RBatisTxExecutor<'_> {
    fn get_rbatis(&self) -> &Rbatis {
        self.rb
    }
}

impl<'a> RBatisTxExecutor<'a> {
    pub async fn begin(&mut self) -> crate::Result<()> {
        return Ok(self.conn.begin().await?);
    }
    pub async fn commit(&mut self) -> crate::Result<()> {
        return Ok(self.conn.commit().await?);
    }
    pub async fn rollback(&mut self) -> crate::Result<()> {
        return Ok(self.conn.rollback().await?);
    }

    pub fn take_conn(self) -> Option<DBPoolConn<'a>> {
        return self.conn.take_conn();
    }
}

impl<'a> Deref for RBatisTxExecutor<'a> {
    type Target = DBTx<'a>;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl<'a> DerefMut for RBatisTxExecutor<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

pub struct RBatisTxExecutorGuard<'a> {
    pub tx: Option<RBatisTxExecutor<'a>>,
    pub callback: Box<dyn FnMut(RBatisTxExecutor<'a>) + Send + 'a>,
}

impl Debug for RBatisTxExecutorGuard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBatisTxExecutorGuard")
            .field("tx", &self.tx)
            .finish()
    }
}

impl<'a, 'b> RBatisTxExecutorGuard<'b> {
    pub fn as_executor(&'a mut self) -> RbatisExecutor<'a, 'b> {
        self.into()
    }

    pub async fn begin(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.begin().await?);
    }

    pub async fn commit(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.commit().await?);
    }

    pub async fn rollback(&mut self) -> crate::Result<()> {
        let mut tx = self.tx.as_mut().ok_or_else(|| Error::from("[rbatis] tx is committed"))?;
        return Ok(tx.rollback().await?);
    }

    pub fn take_conn(mut self) -> Option<DBPoolConn<'b>> {
        match self.tx.take() {
            None => {
                None
            }
            Some(s) => {
                s.take_conn()
            }
        }
    }
}

impl<'a> RBatisTxExecutor<'a> {
    /// defer an func
    /// for example:
    ///     tx.defer(|tx| {});
    ///
    pub fn defer<Call>(self, callback: Call) -> RBatisTxExecutorGuard<'a>
        where Call: 'a + FnMut(Self) + Send {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(callback),
        }
    }

    /// defer and use future method
    /// for example:
    ///         tx.defer_async(|tx| async {
    ///             tx.rollback().await;
    ///         });
    ///
    pub fn defer_async<R, F>(self, mut callback: F) -> RBatisTxExecutorGuard<'a>
        where R: 'a + Future<Output=()>,
              F: 'a + Send + FnMut(RBatisTxExecutor<'a>) -> R {
        RBatisTxExecutorGuard {
            tx: Some(self),
            callback: Box::new(move |arg| {
                block_on(callback(arg));
            }),
        }
    }

    pub async fn fetch_page<T>(
        &self,
        sql: &str,
        args: Vec<bson2::Bson>,
        page_request: &dyn IPageRequest,
    ) -> crate::Result<Page<T>>
        where
            T: DeserializeOwned + Serialize + Send + Sync,
    {
        self.get_rbatis().fetch_page(sql, args, page_request).await
    }
}

impl<'a> Deref for RBatisTxExecutorGuard<'a> {
    type Target = RBatisTxExecutor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.tx.as_ref().unwrap()
    }
}

impl<'a> DerefMut for RBatisTxExecutorGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx.as_mut().unwrap()
    }
}


impl Drop for RBatisTxExecutorGuard<'_> {
    fn drop(&mut self) {
        match self.tx.take() {
            None => {}
            Some(tx) => {
                (self.callback)(tx);
            }
        }
    }
}


#[async_trait]
pub trait Executor: RbatisRef {
    async fn exec(&self, sql: &str, args: Vec<bson2::Bson>) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&self, sql: &str, args: Vec<bson2::Bson>) -> Result<T, Error> where T: DeserializeOwned;
}

#[async_trait]
impl Executor for Rbatis {
    async fn exec(&self, sql: &str, args: Vec<Bson>) -> Result<DBExecResult, Error> {
        self.acquire().await?.exec(sql, args).await
    }

    async fn fetch<T>(&self, sql: &str, args: Vec<Bson>) -> Result<T, Error> where T: DeserializeOwned {
        self.acquire().await?.fetch(sql, args).await
    }
}