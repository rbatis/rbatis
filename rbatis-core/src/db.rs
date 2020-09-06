use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::connection::Connection;
use crate::cursor::Cursor;
use crate::Error;
use crate::executor::Executor;
use crate::mysql::{MySql, MySqlConnection, MySqlCursor, MySqlPool};
use crate::pool::Builder;
use crate::pool::PoolConnection;
use crate::postgres::{PgConnection, PgCursor, PgPool, Postgres};
use crate::query::{Query, query};
use crate::runtime::Mutex;
use crate::sqlite::{Sqlite, SqliteConnection, SqliteCursor, SqlitePool};
use crate::transaction::Transaction;

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
            pool.mysql = Some(MySqlPool::new(driver).await?);
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
            let build = Builder::new()
                .max_size(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_size(opt.min_size)
                .idle_timeout(opt.idle_timeout)
                .test_on_acquire(opt.test_on_acquire);
            pool.mysql = Some(build.build(driver).await?);
        } else if driver.starts_with("postgres") {
            pool.driver_type = DriverType::Postgres;
            let build = Builder::new()
                .max_size(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_size(opt.min_size)
                .idle_timeout(opt.idle_timeout)
                .test_on_acquire(opt.test_on_acquire);
            pool.postgres = Some(build.build(driver).await?);
        } else if driver.starts_with("sqlite") {
            pool.driver_type = DriverType::Sqlite;
            let build = Builder::new()
                .max_size(opt.max_size)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_size(opt.min_size)
                .idle_timeout(opt.idle_timeout)
                .test_on_acquire(opt.test_on_acquire);
            pool.sqlite = Some(build.build(driver).await?);
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
                let q: Query<MySql> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Mysql,
                    mysql: Some(q),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let q: Query<Postgres> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(q),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let q: Query<Sqlite> = query(sql);
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
    pub fn new_sqlite(arg: crate::sqlite::SqliteConnection) -> Self {
        Self {
            driver_type: DriverType::Sqlite,
            mysql: None,
            postgres: None,
            sqlite: Some(arg),
        }
    }
    pub fn new_pg(arg: crate::postgres::PgConnection) -> Self {
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
    pub mysql: Option<Query<'q, MySql>>,
    pub postgres: Option<Query<'q, Postgres>>,
    pub sqlite: Option<Query<'q, Sqlite>>,
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
    pub mysql: Option<PoolConnection<MySqlConnection>>,
    pub postgres: Option<PoolConnection<PgConnection>>,
    pub sqlite: Option<PoolConnection<SqliteConnection>>,
}


impl DBPoolConn {
    pub fn fetch<'q>(&mut self, sql: &'q str) -> crate::Result<DBCursor<'_, 'q>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                });
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
                let data = self.sqlite.as_mut().unwrap().execute(sql).await?;
                return Ok(data);
            }
        }
    }

    pub fn fetch_parperd<'q>(&mut self, sql: DBQuery<'q>) -> crate::Result<DBCursor<'_, 'q>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().fetch(sql.mysql.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().fetch(sql.postgres.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().fetch(sql.sqlite.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                });
            }
        }
    }

    pub async fn execute_parperd(&mut self, sql: DBQuery<'_>) -> crate::Result<u64> {
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
}


pub struct DBCursor<'c, 'q> {
    pub driver_type: DriverType,
    pub mysql: Option<MySqlCursor<'c, 'q>>,
    pub postgres: Option<PgCursor<'c, 'q>>,
    pub sqlite: Option<SqliteCursor<'c, 'q>>,
}

impl<'c, 'q> DBCursor<'c, 'q> {
    /// fetch json and decode json into Type
    pub async fn decode_json<T>(&mut self) -> Result<T, crate::Error>
        where T: DeserializeOwned {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().decode_json().await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().decode_json().await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().decode_json().await?;
                return Ok(data);
            }
        }
    }

    /// only fetch json
    pub async fn fetch_json(&mut self) -> Result<Vec<serde_json::Value>, crate::Error> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().fetch_json().await?;
                return Ok(data);
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().fetch_json().await?;
                return Ok(data);
            }
            &DriverType::Sqlite => {
                let data = self.sqlite.as_mut().unwrap().fetch_json().await?;
                return Ok(data);
            }
        }
    }
}


pub struct DBTx {
    pub driver_type: DriverType,
    pub mysql: Option<Transaction<PoolConnection<MySqlConnection>>>,
    pub postgres: Option<Transaction<PoolConnection<PgConnection>>>,
    pub sqlite: Option<Mutex<Transaction<PoolConnection<SqliteConnection>>>>,
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


    ///TODO find better way reduce the same code
    pub fn fetch<'q>(&mut self, sql: &'q str) -> crate::Result<DBCursor<'_, 'q>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let m = self.sqlite.as_mut().unwrap().get_mut();
                let data = m.fetch(sql);
                return Ok(DBCursor {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                });
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

    pub fn fetch_parperd<'q>(&mut self, sql: DBQuery<'q>) -> crate::Result<DBCursor<'_, 'q>> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data = self.mysql.as_mut().unwrap().fetch(sql.mysql.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Mysql,
                    mysql: Some(data),
                    postgres: None,
                    sqlite: None,
                });
            }
            &DriverType::Postgres => {
                let data = self.postgres.as_mut().unwrap().fetch(sql.postgres.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(data),
                    sqlite: None,
                });
            }
            &DriverType::Sqlite => {
                let tx = self.sqlite.as_mut().unwrap().get_mut();
                let data = tx.fetch(sql.sqlite.unwrap());
                return Ok(DBCursor {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(data),
                });
            }
        }
    }

    pub async fn execute_parperd(&mut self, sql: DBQuery<'_>) -> crate::Result<u64> {
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