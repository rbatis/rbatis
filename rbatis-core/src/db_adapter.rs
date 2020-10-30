use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use sqlx_core::acquire::Acquire;
use sqlx_core::arguments::{Arguments, IntoArguments};
use sqlx_core::connection::Connection;
use sqlx_core::database::Database;
use sqlx_core::done::Done;
use sqlx_core::encode::Encode;
use sqlx_core::executor::Executor;
use sqlx_core::mssql::{Mssql, MssqlArguments, MssqlConnection, MssqlDone, MssqlPool, MssqlRow};
use sqlx_core::mysql::{MySql, MySqlArguments, MySqlConnection, MySqlDone, MySqlPool, MySqlRow};
use sqlx_core::pool::PoolConnection;
use sqlx_core::postgres::{PgArguments, PgConnection, PgConnectOptions, PgDone, PgPool, PgRow, Postgres};
use sqlx_core::query::{Query, query};
use sqlx_core::sqlite::{Sqlite, SqliteArguments, SqliteConnection, SqliteDone, SqlitePool, SqliteRow};
use sqlx_core::transaction::Transaction;
use sqlx_core::types::Type;

use crate::convert::RefJsonCodec;
use crate::db::{DriverType, PoolOptions};
use crate::decode::json_decode;
use crate::Error;
use crate::runtime::Mutex;

#[derive(Debug)]
pub struct DBPool {
    pub driver_type: DriverType,
    pub mysql: Option<MySqlPool>,
    pub postgres: Option<PgPool>,
    pub sqlite: Option<SqlitePool>,
    pub mssql: Option<MssqlPool>,
}


impl DBPool {
    //new with default opt
    pub async fn new(driver: &str) -> crate::Result<DBPool> {
        let mut pool = Self {
            driver_type: DriverType::None,
            mysql: None,
            postgres: None,
            sqlite: None,
            mssql: None,
        };
        if driver.starts_with("mysql") {
            pool.driver_type = DriverType::Mysql;
            let conn = MySqlPool::connect(driver).await;
            if conn.is_err() {
                return Err(crate::Error::from(conn.err().unwrap().to_string()));
            }
            pool.mysql = Some(conn.unwrap());
        } else if driver.starts_with("postgres") {
            pool.driver_type = DriverType::Postgres;
            let conn = PgPool::connect(driver).await;
            if conn.is_err() {
                return Err(crate::Error::from(conn.err().unwrap().to_string()));
            }
            pool.postgres = Some(conn.unwrap());
        } else if driver.starts_with("sqlite") {
            pool.driver_type = DriverType::Sqlite;
            let conn = SqlitePool::connect(driver).await;
            if conn.is_err() {
                return Err(crate::Error::from(conn.err().unwrap().to_string()));
            }
            pool.sqlite = Some(conn.unwrap());
        } else if driver.starts_with("mssql") || driver.starts_with("sqlserver") {
            pool.driver_type = DriverType::Mssql;
            let conn = MssqlPool::connect(driver).await;
            if conn.is_err() {
                return Err(crate::Error::from(conn.err().unwrap().to_string()));
            }
            pool.mssql = Some(conn.unwrap());
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
            mssql: None,
        };
        if driver.starts_with("mysql") {
            pool.driver_type = DriverType::Mysql;
            let build = sqlx_core::pool::PoolOptions::<MySql>::default()
                .max_connections(opt.max_connections)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_connections)
                .idle_timeout(opt.idle_timeout)
                .test_before_acquire(opt.test_before_acquire);
            let p = build.connect_lazy(driver);
            if p.is_err() {
                return Err(crate::Error::from(p.err().unwrap().to_string()));
            }
            pool.mysql = Some(p.unwrap());
        } else if driver.starts_with("postgres") {
            pool.driver_type = DriverType::Postgres;
            let build = sqlx_core::pool::PoolOptions::<Postgres>::new()
                .max_connections(opt.max_connections)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_connections)
                .idle_timeout(opt.idle_timeout)
                .test_before_acquire(opt.test_before_acquire);
            let p = build.connect_lazy(driver);
            if p.is_err() {
                return Err(crate::Error::from(p.err().unwrap().to_string()));
            }
            pool.postgres = Some(p.unwrap());
        } else if driver.starts_with("sqlite") {
            pool.driver_type = DriverType::Sqlite;
            let build = sqlx_core::pool::PoolOptions::<Sqlite>::new()
                .max_connections(opt.max_connections)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_connections)
                .idle_timeout(opt.idle_timeout)
                .test_before_acquire(opt.test_before_acquire);
            let p = build.connect_lazy(driver);
            if p.is_err() {
                return Err(crate::Error::from(p.err().unwrap().to_string()));
            }
            pool.sqlite = Some(p.unwrap());
        } else if driver.starts_with("mssql") || driver.starts_with("sqlserver") {
            pool.driver_type = DriverType::Mssql;
            let build = sqlx_core::pool::PoolOptions::<Mssql>::new()
                .max_connections(opt.max_connections)
                .max_lifetime(opt.max_lifetime)
                .connect_timeout(opt.connect_timeout)
                .min_connections(opt.min_connections)
                .idle_timeout(opt.idle_timeout)
                .test_before_acquire(opt.test_before_acquire);
            let p = build.connect_lazy(driver);
            if p.is_err() {
                return Err(crate::Error::from(p.err().unwrap().to_string()));
            }
            pool.mssql = Some(p.unwrap());
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
                    mssql: None,
                });
            }
            &DriverType::Postgres => {
                let q: Query<Postgres, PgArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(q),
                    sqlite: None,
                    mssql: None,
                });
            }
            &DriverType::Sqlite => {
                let q: Query<Sqlite, SqliteArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(q),
                    mssql: None,
                });
            }
            &DriverType::Mssql => {
                let q: Query<Mssql, MssqlArguments> = query(sql);
                return Ok(DBQuery {
                    driver_type: DriverType::Mssql,
                    mysql: None,
                    postgres: None,
                    sqlite: None,
                    mssql: Some(q),
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
                let conn = self.mysql.as_ref().unwrap().acquire().await;
                if conn.is_err() {
                    return Err(crate::Error::from(conn.err().unwrap().to_string()));
                }
                return Ok(DBPoolConn {
                    driver_type: DriverType::Mysql,
                    mysql: Some(conn.unwrap()),
                    postgres: None,
                    sqlite: None,
                    mssql: None,
                });
            }
            &DriverType::Postgres => {
                let conn = self.postgres.as_ref().unwrap().acquire().await;
                if conn.is_err() {
                    return Err(crate::Error::from(conn.err().unwrap().to_string()));
                }
                return Ok(DBPoolConn {
                    driver_type: DriverType::Postgres,
                    mysql: None,
                    postgres: Some(conn.unwrap()),
                    sqlite: None,
                    mssql: None,
                });
            }
            &DriverType::Sqlite => {
                let conn = self.sqlite.as_ref().unwrap().acquire().await;
                if conn.is_err() {
                    return Err(crate::Error::from(conn.err().unwrap().to_string()));
                }
                return Ok(DBPoolConn {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(conn.unwrap()),
                    mssql: None,
                });
            }
            &DriverType::Mssql => {
                let conn = self.mssql.as_ref().unwrap().acquire().await;
                if conn.is_err() {
                    return Err(crate::Error::from(conn.err().unwrap().to_string()));
                }
                return Ok(DBPoolConn {
                    driver_type: DriverType::Sqlite,
                    mysql: None,
                    postgres: None,
                    sqlite: None,
                    mssql: Some(conn.unwrap()),
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
                    driver_type: self.driver_type,
                    mysql: Some(conn.unwrap()),
                    postgres: None,
                    sqlite: None,
                    mssql: None,
                }));
            }
            &DriverType::Postgres => {
                let conn = self.postgres.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: self.driver_type,
                    mysql: None,
                    postgres: Some(conn.unwrap()),
                    sqlite: None,
                    mssql: None,
                }));
            }
            &DriverType::Sqlite => {
                let conn = self.sqlite.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: self.driver_type,
                    mysql: None,
                    postgres: None,
                    sqlite: Some(conn.unwrap()),
                    mssql: None,
                }));
            }
            &DriverType::Mssql => {
                let conn = self.mssql.as_ref().unwrap().try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn {
                    driver_type: self.driver_type,
                    mysql: None,
                    postgres: None,
                    sqlite: None,
                    mssql: Some(conn.unwrap()),
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
                    driver_type: self.driver_type,
                    mysql: Some(convert_result(self.mysql.as_ref().unwrap().begin().await)?),
                    postgres: None,
                    sqlite: None,
                    mssql: None,
                })
            }
            &DriverType::Postgres => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    postgres: Some(convert_result(self.postgres.as_ref().unwrap().begin().await)?),
                    mysql: None,
                    sqlite: None,
                    mssql: None,
                })
            }
            &DriverType::Sqlite => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    sqlite: Some(Mutex::new(convert_result(self.sqlite.as_ref().unwrap().begin().await)?)),
                    postgres: None,
                    mysql: None,
                    mssql: None,
                })
            }
            &DriverType::Mssql => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    mssql: Some(convert_result(self.mssql.as_ref().unwrap().begin().await)?),
                    mysql: None,
                    postgres: None,
                    sqlite: None,
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
    pub sqlite: Option<Query<'q, Sqlite, SqliteArguments<'q>>>,
    pub mssql: Option<Query<'q, Mssql, MssqlArguments>>,
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
                        q = q.bind(Some(s.to_owned()));
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
                        q = q.bind(Option::Some(b.to_owned()));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string()));
                    }
                }
                self.mysql = Some(q);
            }
            &DriverType::Postgres => {
                let mut q = self.postgres.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s.to_owned()));
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
                        q = q.bind(Option::Some(b.to_owned()));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string()));
                    }
                }
                self.postgres = Some(q);
            }
            &DriverType::Sqlite => {
                let mut q = self.sqlite.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s.to_owned()));
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
                        q = q.bind(Option::Some(b.to_owned()));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string()));
                    }
                }
                self.sqlite = Some(q);
            }
            &DriverType::Mssql => {
                let mut q = self.mssql.take().unwrap();
                match t {
                    serde_json::Value::String(s) => {
                        q = q.bind(Some(s.to_owned()));
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
                        q = q.bind(Option::Some(b.to_owned()));
                    }
                    _ => {
                        q = q.bind(Some(t.to_string()));
                    }
                }
                self.mssql = Some(q);
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
    pub mssql: Option<PoolConnection<Mssql>>,
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
            &DriverType::Mssql => {
                if self.mssql.is_none() {
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
                let async_stream: Vec<MySqlRow> = convert_result(self.mysql.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = async_stream.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let async_stream: Vec<PgRow> = convert_result(self.postgres.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = async_stream.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let data: Vec<SqliteRow> = convert_result(self.sqlite.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Mssql => {
                let async_stream: Vec<MssqlRow> = convert_result(self.mssql.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = async_stream.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn execute(&mut self, sql: &str) -> crate::Result<DBExecResult> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data: MySqlDone = convert_result(self.mysql.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Postgres => {
                let data: PgDone = convert_result(self.postgres.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Sqlite => {
                let data: SqliteDone = convert_result(self.sqlite.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Mssql => {
                let data: MssqlDone = convert_result(self.mssql.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
        }
    }

    pub async fn fetch_parperd<T>(&mut self, sql: DBQuery<'_>) -> crate::Result<(T, usize)>
        where T: DeserializeOwned {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data: Vec<MySqlRow> = convert_result(self.mysql.as_mut().unwrap().fetch_all(sql.mysql.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let data: Vec<PgRow> = convert_result(self.postgres.as_mut().unwrap().fetch_all(sql.postgres.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let data: Vec<SqliteRow> = convert_result(self.sqlite.as_mut().unwrap().fetch_all(sql.sqlite.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Mssql => {
                let data: Vec<MssqlRow> = convert_result(self.mssql.as_mut().unwrap().fetch_all(sql.mssql.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<DBExecResult> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let result: MySqlDone = convert_result(self.mysql.as_mut().unwrap().execute(sql.mysql.unwrap()).await)?;
                return Ok(DBExecResult::from(result));
            }
            &DriverType::Postgres => {
                let data: PgDone = convert_result(self.postgres.as_mut().unwrap().execute(sql.postgres.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Sqlite => {
                let data: SqliteDone = convert_result(self.sqlite.as_mut().unwrap().execute(sql.sqlite.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Mssql => {
                let data: MssqlDone = convert_result(self.mssql.as_mut().unwrap().execute(sql.mssql.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
        }
    }

    pub async fn begin(&'static mut self) -> crate::Result<DBTx> {
        self.check_alive()?;
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    mysql: Some(convert_result(self.mysql.as_mut().unwrap().begin().await)?),
                    postgres: None,
                    sqlite: None,
                    mssql: None,
                })
            }
            &DriverType::Postgres => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    postgres: Some(convert_result(self.postgres.as_mut().unwrap().begin().await)?),
                    mysql: None,
                    sqlite: None,
                    mssql: None,
                })
            }
            &DriverType::Sqlite => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    sqlite: Some(Mutex::new(convert_result(self.sqlite.as_mut().unwrap().begin().await)?)),
                    postgres: None,
                    mysql: None,
                    mssql: None,
                })
            }
            &DriverType::Mssql => {
                Ok(DBTx {
                    driver_type: self.driver_type,
                    mssql: Some(convert_result(self.mssql.as_mut().unwrap().begin().await)?),
                    mysql: None,
                    sqlite: None,
                    postgres: None,
                })
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
                return Ok(convert_result(self.mysql.as_mut().unwrap().ping().await)?);
            }
            &DriverType::Postgres => {
                return Ok(convert_result(self.postgres.as_mut().unwrap().ping().await)?);
            }
            &DriverType::Sqlite => {
                return Ok(convert_result(self.sqlite.as_mut().unwrap().ping().await)?);
            }
            &DriverType::Mssql => {
                return Ok(convert_result(self.mssql.as_mut().unwrap().ping().await)?);
            }
        }
    }

    pub async fn close(mut self) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                self.mysql = None;
                return Ok(());
            }
            &DriverType::Postgres => {
                self.postgres = None;
                return Ok(());
            }
            &DriverType::Sqlite => {
                self.sqlite = None;
                return Ok(());
            }
            &DriverType::Mssql => {
                self.mssql = None;
                return Ok(());
            }
        }
    }
}


pub struct DBTx {
    pub driver_type: DriverType,
    pub mysql: Option<Transaction<'static, MySql>>,
    pub postgres: Option<Transaction<'static, Postgres>>,
    pub sqlite: Option<Mutex<Transaction<'static, Sqlite>>>,
    pub mssql: Option<Transaction<'static, Mssql>>,
}


impl DBTx {
    pub async fn commit(&mut self) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                convert_result(self.mysql.take().unwrap().commit().await)
            }
            &DriverType::Postgres => {
                convert_result(self.postgres.take().unwrap().commit().await)
            }
            &DriverType::Sqlite => {
                convert_result(self.sqlite.take().unwrap().into_inner().commit().await)
            }
            &DriverType::Mssql => {
                convert_result(self.mssql.take().unwrap().commit().await)
            }
        }
    }

    pub async fn rollback(mut self) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                convert_result(self.mysql.take().unwrap().rollback().await)
            }
            &DriverType::Postgres => {
                convert_result(self.postgres.take().unwrap().rollback().await)
            }
            &DriverType::Sqlite => {
                convert_result(self.sqlite.take().unwrap().into_inner().rollback().await)
            }
            &DriverType::Mssql => {
                convert_result(self.mssql.take().unwrap().rollback().await)
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
                let data: Vec<MySqlRow> = convert_result(self.mysql.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let data: Vec<PgRow> = convert_result(self.postgres.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let data: Vec<SqliteRow> = convert_result(self.sqlite.as_mut().unwrap().lock().await.fetch_all(sql).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Mssql => {
                let data: Vec<MssqlRow> = convert_result(self.mssql.as_mut().unwrap().fetch_all(sql).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
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
                let data: Vec<MySqlRow> = convert_result(self.mysql.as_mut().unwrap().fetch_all(sql.mysql.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Postgres => {
                let data: Vec<PgRow> = convert_result(self.postgres.as_mut().unwrap().fetch_all(sql.postgres.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Sqlite => {
                let data: Vec<SqliteRow> = convert_result(self.sqlite.as_mut().unwrap().lock().await.fetch_all(sql.sqlite.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
            &DriverType::Mssql => {
                let data: Vec<MssqlRow> = convert_result(self.mssql.as_mut().unwrap().fetch_all(sql.mssql.unwrap()).await)?;
                let json_array = data.try_to_json()?.as_array().unwrap().to_owned();
                let return_len = json_array.len();
                let result = json_decode::<T>(json_array)?;
                Ok((result, return_len))
            }
        }
    }

    pub async fn execute(&mut self, sql: &str) -> crate::Result<DBExecResult> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data: MySqlDone = convert_result(self.mysql.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Postgres => {
                let data: PgDone = convert_result(self.postgres.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Sqlite => {
                let data: SqliteDone = convert_result(self.sqlite.as_mut().unwrap().lock().await.execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Mssql => {
                let data: MssqlDone = convert_result(self.mssql.as_mut().unwrap().execute(sql).await)?;
                return Ok(DBExecResult::from(data));
            }
        }
    }


    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<DBExecResult> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                let data: MySqlDone = convert_result(self.mysql.as_mut().unwrap().execute(sql.mysql.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Postgres => {
                let data: PgDone = convert_result(self.postgres.as_mut().unwrap().execute(sql.postgres.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Sqlite => {
                let data: SqliteDone = convert_result(self.sqlite.as_mut().unwrap().lock().await.execute(sql.sqlite.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
            &DriverType::Mssql => {
                let data: MssqlDone = convert_result(self.mssql.as_mut().unwrap().execute(sql.mssql.unwrap()).await)?;
                return Ok(DBExecResult::from(data));
            }
        }
    }
}

pub fn convert_result<T>(arg: Result<T, sqlx_core::error::Error>) -> crate::Result<T> {
    if arg.is_err() {
        return Err(crate::Error::from(arg.err().unwrap().to_string()));
    }
    return Ok(arg.unwrap());
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBExecResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

impl From<MySqlDone> for DBExecResult {
    fn from(arg: MySqlDone) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: Some(arg.last_insert_id() as i64),
        }
    }
}

impl From<PgDone> for DBExecResult {
    fn from(arg: PgDone) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: None,
        }
    }
}

impl From<SqliteDone> for DBExecResult {
    fn from(arg: SqliteDone) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: Some(arg.last_insert_rowid()),
        }
    }
}

impl From<MssqlDone> for DBExecResult {
    fn from(arg: MssqlDone) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: None,
        }
    }
}