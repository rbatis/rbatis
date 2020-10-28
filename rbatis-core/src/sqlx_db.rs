use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use sqlx_core::connection::Connection;
use sqlx_core::cursor::Cursor;
use sqlx_core::database::Database;
use sqlx_core::encode::Encode;
use sqlx_core::executor::Executor;
use sqlx_core::mysql::{MySql, MySqlArguments, MySqlConnection, MySqlConnectOptions, MySqlPool, MySqlRow};
use sqlx_core::pool::PoolConnection;
use sqlx_core::postgres::{PgConnection, PgConnectOptions, PgPool, Postgres, PgRow};
use sqlx_core::query::{Query, query};
use sqlx_core::sqlite::{Sqlite, SqliteConnection, SqliteConnectOptions, SqlitePool, SqliteRow};
use sqlx_core::transaction::Transaction;
use sqlx_core::types::Type;

use crate::decode::json_decode;
use crate::Error;
use crate::postgres::PgArguments;
use crate::runtime::Mutex;
use crate::sqlite::SqliteArguments;
use crate::convert::RefJsonCodec;

#[derive(Debug, Clone, Copy)]
pub struct PoolOptions {
    pub max_size: u32,
    pub connect_timeout: Duration,
    pub min_size: u32,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub test_on_acquire: bool,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            // pool a maximum of 10 connections to the same database
            max_size: 10,
            // don't open connections until necessary
            min_size: 0,
            // try to connect for 10 seconds before erroring
            connect_timeout: Duration::from_secs(60),
            // reap connections that have been alive > 30 minutes
            // prevents unbounded live-leaking of memory due to naive prepared statement caching
            // see src/cache.rs for context
            max_lifetime: Some(Duration::from_secs(1800)),
            // don't reap connections based on idle time
            idle_timeout: None,
            // If true, test the health of a connection on acquire
            test_on_acquire: true,
        }
    }
}

impl PoolOptions {
    pub fn new() -> Self {
        PoolOptions::default()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum DriverType {
    None = 0,
    Mysql = 1,
    Postgres = 2,
    Sqlite = 3,
}

#[derive(Debug)]
pub struct DBPool {
    pub driver_type: DriverType,
    pub mysql: Option<MySqlPool>,
    pub postgres: Option<PgPool>,
    pub sqlite: Option<SqlitePool>,
}


impl DBPool {
    //new with default opt
    pub async fn new(driver: &str) -> crate::Result<DBPool> {
        let mut pool = Self {
            driver_type: DriverType::None,
            mysql: None,
            postgres: None,
            sqlite: None,
        };
        if driver.starts_with("mysql") {
            pool.driver_type = DriverType::Mysql;
            pool.mysql = Some(MySqlPool::connect(driver).await?);
        } else if driver.starts_with("postgres") {
            pool.driver_type = DriverType::Postgres;
            pool.postgres = Some(PgPool::new(driver).await?);
        } else if driver.starts_with("sqlite") {
            pool.driver_type = DriverType::Sqlite;
            pool.sqlite = Some(SqlitePool::new(driver).await?);
        } else {
            return Err(Error::from("unsupport driver type!"));
        }
        return Ok(pool);
    }

    //new_opt
    pub async fn new_opt(driver: &str, opt: &PoolOptions) -> crate::Result<DBPool> {
        let mut pool = Self {
            driver_type: DriverType::None,
            mysql: None,
            postgres: None,
            sqlite: None,
        };
        if driver.starts_with("mysql") {
            pool.driver_type = DriverType::Mysql;
            let build = MySqlConnectOptions::<MySql>::new()
                .max_connections(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_size)
                .idle_timeout(opt.idle_timeout);
            pool.mysql = Some(MySqlPool::connect_with(build).await?);
        } else if driver.starts_with("postgres") {
            pool.driver_type = DriverType::Postgres;
            let build = PgConnectOptions::new()
                .max_connections(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_size)
                .idle_timeout(opt.idle_timeout);
            pool.postgres = Some(PgPool::connect_with(build).await?);
        } else if driver.starts_with("sqlite") {
            pool.driver_type = DriverType::Sqlite;
            let build = SqliteConnectOptions::new()
                .max_connections(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_size)
                .idle_timeout(opt.idle_timeout);
            pool.sqlite = Some(SqlitePool::connect_with(build).await?);
        } else {
            return Err(Error::from("unsupport driver type!"));
        }
        return Ok(pool);
    }

    pub fn make_query<'f, 's>(&'f self, sql: &'s str) -> crate::Result<DBQuery<'s>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let q: Query<MySql, MySqlArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Mysql,
                    mysql: Some(q),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let q: Query<Postgres, PgArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(q),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let q: Query<Sqlite, SqliteArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(q),
                });
            }
        }
    }
    /// Retrieves a connection from the pool.
    ///
    /// Waits for at most the configured connection timeout before returning an error.
    pub async fn acquire(&self) -> crate::Result<DBPoolConn> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let conn = self.mysql.as_ref().unwrap().acquire().await?;
                return Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: Some(conn),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let conn = self.postgres.as_ref().unwrap().acquire().await?;
                return Ok(DBPoolConn {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(conn),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let conn = self.sqlite.as_ref().unwrap().acquire().await?;
                return Ok(DBPoolConn {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(conn),
                });
            }
        }
    }

    /// Attempts to retrieve a connection from the pool if there is one available.
    ///
    /// Returns `None` immediately if there are no idle connections available in the pool.
    pub fn try_acquire(&self) -> crate::Result<Option<DBPoolConn>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let conn = self.mysql.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: Some(conn.unwrap()),
                    postgres: None,
                    sqlite: None,
                }));
            }
            &DriverType::Postgres => {
                let conn = self.postgres.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(conn.unwrap()),
                    sqlite: None,
                }));
            }
            &DriverType::Sqlite => {
                let conn = self.sqlite.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(conn.unwrap()),
                }));
            }
        }
    }

    pub async fn begin(&self) -> crate::Result<DBTx> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                Ok(DBTx {
                    driver_type: DriverType::Mysql,
                    mysql: Some(self.mysql.as_ref().unwrap().begin().await?),
                    postgres: None,
                    sqlite: None,
                })
            }
            &DriverType::Postgres => {
                Ok(DBTx {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(self.postgres.as_ref().unwrap().begin().await?),
                    sqlite: None,
                })
            }
            &DriverType::Sqlite => {
                Ok(DBTx {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(Mutex::new(self.sqlite.as_ref().unwrap().begin().await?)),
                })
            }
        }
    }
}

pub struct DBConnection {
    pub driver_type: DriverType,
    pub mysql: Option<MySqlConnection>,
    pub postgres: Option<PgConnection>,
    pub sqlite: Option<SqliteConnection>,
}

impl DBConnection {
    pub fn new_my(arg: MySqlConnection) -> Self {
        Self {
            driver_type: DriverType::Mysql,
            mysql: Some(arg),
            postgres: None,
            sqlite: None,
        }
    }
    pub fn new_sqlite(arg: SqliteConnection) -> Self {
        Self {
            driver_type: DriverType::Sqlite,
            mysql: None,
            postgres: None,
            sqlite: Some(arg),
        }
    }
    pub fn new_pg(arg: PgConnection) -> Self {
        Self {
            driver_type: DriverType::Postgres,
            mysql: None,
            postgres: Some(arg),
            sqlite: None,
        }
    }
}


pub struct DBQuery<'q> {
    pub driver_type: DriverType,
    pub mysql: Option<Query<'q, MySql, MySqlArguments>>,
    pub postgres: Option<Query<'q, Postgres, PgArguments>>,
    pub sqlite: Option<Query<'q, Sqlite, SqliteArguments>>,
}

impl<'q> DBQuery<'q> {
    pub fn bind_value(&mut self, t: &serde_json::Value) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let mut q = self.mysql.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s));
                    }
                    serde_json::Value::Null => {
                        q = q.bind(Option::<String>::None);
                    }
                    serde_json::Value::Number(n) => {
                        if n.is_f64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_u64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_i64() {
                            q = q.bind(n.as_i64().unwrap());
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        q = q.bind(Option::Some(b));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string().as_str()));
                    }
                }
                self.mysql = Some(q);
            }
            &DriverType::Postgres => {
                let mut q = self.postgres.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s));
                    }
                    serde_json::Value::Null => {
                        q = q.bind(Option::<String>::None);
                    }
                    serde_json::Value::Number(n) => {
                        if n.is_f64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_u64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_i64() {
                            q = q.bind(n.as_i64().unwrap());
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        q = q.bind(Option::Some(b));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string().as_str()));
                    }
                }
                self.postgres = Some(q);
            }
            &DriverType::Sqlite => {
                let mut q = self.sqlite.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s));
                    }
                    serde_json::Value::Null => {
                        q = q.bind(Option::<String>::None);
                    }
                    serde_json::Value::Number(n) => {
                        if n.is_f64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_u64() {
                            q = q.bind(n.as_f64().unwrap());
                        } else if n.is_i64() {
                            q = q.bind(n.as_i64().unwrap());
                        }
                    }
                    serde_json::Value::Bool(b) => {
                        q = q.bind(Option::Some(b));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string().as_str()));
                    }
                }
                self.sqlite = Some(q);
            }
        }
        return Ok(());
    }
}


pub struct DBPoolConn {
    pub driver_type: DriverType,
    pub mysql: Option<PoolConnection<MySql>>,
    pub postgres: Option<PoolConnection<Postgres>>,
    pub sqlite: Option<PoolConnection<Sqlite>>,
}


impl DBPoolConn {
    pub fn check_alive(&self) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                if self.mysql.is_none() {
                    return Err(Error::from("un init DBPoolConn!"));
                }
            }
            &DriverType::Postgres => {
                if self.postgres.is_none() {
                    return Err(Error::from("un init DBPoolConn!"));
                }
            }
            &DriverType::Sqlite => {
                if self.sqlite.is_none() {
                    return Err(Error::from("un init DBPoolConn!"));
                }
            }
        }

        return Ok(());
    }

    pub async fn fetch<'q, T>(&mut self, sql: &'q str) -> crate::Result<(T, usize)>
        where T: DeserializeOwned {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let async_stream: Vec<MySqlRow> = self.mysql.as_mut().unwrap().fetch_all(sql).await?;
                let json_array=async_stream.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let async_stream: Vec<PgRow> = self.postgres.as_mut().unwrap().fetch_all(sql).await?;
                let json_array=async_stream.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let mut data: Vec<SqliteRow> = self.sqlite.as_mut().unwrap().fetch_all(sql).await?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn execute(&mut self, sql: &str) -> crate::Result<u64> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
        }
    }

    pub async fn fetch_parperd<'q, T>(&mut self, sql: DBQuery<'q>) -> crate::Result<(T, usize)>
        where T: DeserializeOwned {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let mut data: Vec<MySqlRow> = self.mysql.as_mut().unwrap().fetch_all(sql.mysql.unwrap()).await?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let mut data: Vec<PgRow> = self.postgres.as_mut().unwrap().fetch_all(sql.postgres.unwrap()).await?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let mut data: Vec<SqliteRow> = self.sqlite.as_mut().unwrap().fetch_all(sql.sqlite.unwrap()).await?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<u64> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().execute(sql.mysql.unwrap()).await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().execute(sql.postgres.unwrap()).await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().execute(sql.sqlite.unwrap()).await?;
                return Ok(data);
            }
        }
    }

    pub async fn begin(self) -> crate::Result<DBTx> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.unwrap().begin().await?;
                return Ok(DBTx {
                    driver_type: self.driver_type,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let data = self.postgres.unwrap().begin().await?;
                return Ok(DBTx {
                    driver_type: self.driver_type,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.unwrap().begin().await?;
                return Ok(DBTx {
                    driver_type: self.driver_type,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(Mutex::new(data)),
                });
            }
        }
    }

    pub async fn ping(&mut self) -> crate::Result<()> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                return Ok(self.mysql.as_mut().unwrap().ping().await?);
            }
            &DriverType::Postgres => {
                return Ok(self.postgres.as_mut().unwrap().ping().await?);
            }
            &DriverType::Sqlite => {
                return Ok(self.sqlite.as_mut().unwrap().ping().await?);
            }
        }
    }

    pub async fn close(mut self) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                if self.mysql.is_none() {
                    return Ok(());
                }
                return Ok(self.mysql.take().unwrap().close().await?);
            }
            &DriverType::Postgres => {
                if self.postgres.is_none() {
                    return Ok(());
                }
                return Ok(self.postgres.take().unwrap().close().await?);
            }
            &DriverType::Sqlite => {
                if self.sqlite.is_none() {
                    return Ok(());
                }
                return Ok(self.sqlite.take().unwrap().close().await?);
            }
        }
    }
}


pub struct DBTx {
    pub driver_type: DriverType,
    pub mysql: Option<Transaction<MySql>>,
    pub postgres: Option<Transaction<Postgres>>,
    pub sqlite: Option<Mutex<Transaction<Sqlite>>>,
}

impl DBTx {
    pub async fn commit(&mut self) -> crate::Result<DBPoolConn> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.take().unwrap().commit().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                })
            }
            &DriverType::Postgres => {
                let data = self.postgres.take().unwrap().commit().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                })
            }
            &DriverType::Sqlite => {
                let take = self.sqlite.take().unwrap().into_inner();
                let data = take.commit().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                })
            }
        }
    }

    pub async fn rollback(&mut self) -> crate::Result<DBPoolConn> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.take().unwrap().rollback().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                })
            }
            &DriverType::Postgres => {
                let data = self.postgres.take().unwrap().rollback().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                })
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.take().unwrap().into_inner().rollback().await?;
                Ok(DBPoolConn {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                })
            }
        }
    }

    pub async fn fetch<'q, T>(&mut self, sql: &'q str) -> crate::Result<(T, usize)>
        where T: DeserializeOwned {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let mut data: MySqlCursor = self.mysql.as_mut().unwrap().cursor(sql);
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let mut data: PgCursor = self.postgres.as_mut().unwrap().cursor(sql);
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let mut m = self.sqlite.as_mut().unwrap().lock().await;
                let mut data: SqliteCursor = m.cursor(sql);
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn fetch_parperd<'q, T>(&mut self, sql: DBQuery<'q>) -> crate::Result<(T, usize)>
        where T: DeserializeOwned {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let mut data: MySqlCursor = self.mysql.as_mut().unwrap().cursor(sql.mysql.unwrap());
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let mut data: PgCursor = self.postgres.as_mut().unwrap().cursor(sql.postgres.unwrap());
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let mut mutex_guard = self.sqlite.as_mut().unwrap().lock().await;
                let mut data: SqliteCursor = mutex_guard.cursor(sql.sqlite.unwrap());
                let json_array = data.fetch_json().await?;
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn execute(&mut self, sql: &str) -> crate::Result<u64> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().lock().await.execute(sql).await?;
                return Ok(data);
            }
        }
    }


    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<u64> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().execute(sql.mysql.unwrap()).await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().execute(sql.postgres.unwrap()).await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().lock().await.execute(sql.sqlite.unwrap()).await?;
                return Ok(data);
            }
        }
    }
}