#![allow(unreachable_patterns)]

use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use sqlx_core::acquire::Acquire;
use sqlx_core::arguments::{Arguments, IntoArguments};
use sqlx_core::connection::{Connection, ConnectOptions};
use sqlx_core::database::Database;
use sqlx_core::encode::Encode;
use sqlx_core::executor::Executor;
#[cfg(feature = "mssql")]
use sqlx_core::mssql::{
    Mssql, MssqlArguments, MssqlConnection, MssqlConnectOptions, MssqlPool, MssqlQueryResult, MssqlRow,
};
#[cfg(feature = "mysql")]
use sqlx_core::mysql::{
    MySql, MySqlArguments, MySqlConnection, MySqlConnectOptions, MySqlPool, MySqlQueryResult, MySqlRow,
    MySqlSslMode,
};
use sqlx_core::pool::{PoolConnection, Pool};
#[cfg(feature = "postgres")]
use sqlx_core::postgres::{
    PgArguments, PgConnection, PgConnectOptions, PgPool, PgPoolOptions, PgQueryResult, PgRow, PgSslMode,
    Postgres,
};
use sqlx_core::query::{query, Query};
#[cfg(feature = "sqlite")]
use sqlx_core::sqlite::{
    Sqlite, SqliteArguments, SqliteConnection, SqliteConnectOptions, SqlitePool, SqliteQueryResult,
    SqliteRow,
};
use sqlx_core::transaction::Transaction;
use sqlx_core::types::Type;

use crate::convert::{RefJsonCodec, ResultCodec};
use crate::db::{DBPoolOptions, DriverType};
use crate::decode::decode;
use crate::Error;
use crate::Result;
use std::ops::DerefMut;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Local, Utc};
use bigdecimal_::BigDecimal;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use crate::types::TimestampZ;

/// DataDecoder Process some bson data not yet supported by the framework, which returns TypeInfo and bytes
pub trait DataDecoder: Debug+Sync+Send {
    fn decode(&self, key: &str, data: &mut Bson) -> crate::Result<()>;
}


#[derive(Debug, Clone)]
pub enum DBPool {
    None,
    #[cfg(feature = "mysql")]
    Mysql(MySqlPool, Arc<Box<dyn DataDecoder>>),
    #[cfg(feature = "postgres")]
    Postgres(PgPool, Arc<Box<dyn DataDecoder>>),
    #[cfg(feature = "sqlite")]
    Sqlite(SqlitePool, Arc<Box<dyn DataDecoder>>),
    #[cfg(feature = "mssql")]
    Mssql(MssqlPool, Arc<Box<dyn DataDecoder>>),
}

impl DBPool {
    pub fn driver_type(&self) -> DriverType {
        match self {
            DBPool::None => { DriverType::None }
            #[cfg(feature = "mysql")]
            DBPool::Mysql(_, _) => { DriverType::Mysql }
            #[cfg(feature = "postgres")]
            DBPool::Postgres(_, _) => { DriverType::Postgres }
            #[cfg(feature = "sqlite")]
            DBPool::Sqlite(_, _) => { DriverType::Sqlite }
            #[cfg(feature = "mssql")]
            DBPool::Mssql(_, _) => { DriverType::Mssql }
        }
    }

    //new with default opt
    pub async fn new(driver: &str) -> crate::Result<DBPool> {
        return Self::new_opt_str(driver, DBPoolOptions::default()).await;
    }

    //new with str
    pub async fn new_opt_str(driver: &str, opt: DBPoolOptions) -> crate::Result<DBPool> {
        let conn_opt = DBConnectOption::from(driver)?;
        return Self::new_opt(&conn_opt, opt).await;
    }

    //new_opt from DBConnectionOption option and PoolOptions
    pub async fn new_opt(driver: &DBConnectOption, opt: DBPoolOptions) -> crate::Result<DBPool> {
        let mut pool = DBPool::None;
        match &driver.driver_type {
            #[cfg(feature = "mysql")]
            DriverType::Mysql => {
                let build = sqlx_core::pool::PoolOptions::<MySql>::default()
                    .max_connections(opt.max_connections)
                    .max_lifetime(opt.max_lifetime)
                    .connect_timeout(opt.connect_timeout)
                    .min_connections(opt.min_connections)
                    .idle_timeout(opt.idle_timeout)
                    .test_before_acquire(opt.test_before_acquire);
                let p = build.connect_with(driver.mysql.clone().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?).await?;
                pool = DBPool::Mysql(p, Arc::new(opt.decoder));
                return Ok(pool);
            }
            #[cfg(feature = "postgres")]
            DriverType::Postgres => {
                let build = sqlx_core::pool::PoolOptions::<Postgres>::new()
                    .max_connections(opt.max_connections)
                    .max_lifetime(opt.max_lifetime)
                    .connect_timeout(opt.connect_timeout)
                    .min_connections(opt.min_connections)
                    .idle_timeout(opt.idle_timeout)
                    .test_before_acquire(opt.test_before_acquire);
                let p = build.connect_with(driver.postgres.clone().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?).await?;
                pool = DBPool::Postgres(p, Arc::new(opt.decoder));
                return Ok(pool);
            }
            #[cfg(feature = "sqlite")]
            DriverType::Sqlite => {
                let build = sqlx_core::pool::PoolOptions::<Sqlite>::new()
                    .max_connections(opt.max_connections)
                    .max_lifetime(opt.max_lifetime)
                    .connect_timeout(opt.connect_timeout)
                    .min_connections(opt.min_connections)
                    .idle_timeout(opt.idle_timeout)
                    .test_before_acquire(opt.test_before_acquire);
                let p = build.connect_with(driver.sqlite.clone().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?).await?;
                pool = DBPool::Sqlite(p, Arc::new(opt.decoder));
                return Ok(pool);
            }
            #[cfg(feature = "mssql")]
            DriverType::Mssql => {
                let build = sqlx_core::pool::PoolOptions::<Mssql>::new()
                    .max_connections(opt.max_connections)
                    .max_lifetime(opt.max_lifetime)
                    .connect_timeout(opt.connect_timeout)
                    .min_connections(opt.min_connections)
                    .idle_timeout(opt.idle_timeout)
                    .test_before_acquire(opt.test_before_acquire);
                let p = build.connect_with(driver.mssql.clone().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?).await?;
                pool = DBPool::Mssql(p, Arc::new(opt.decoder));
                return Ok(pool);
            }
            _ => {
                return Err(Error::from(
                    "unsupport driver type or not enable target database feature!",
                ));
            }
        }
    }


    pub fn make_query<'f, 's>(&'f self, sql: &'s str) -> crate::Result<DBQuery<'s>> {
        return self.driver_type().make_db_query(sql);
    }
    /// Retrieves a connection from the pool.
    ///
    /// Waits for at most the configured connection timeout before returning an error.
    pub async fn acquire(&self) -> crate::Result<DBPoolConn<'_>> {
        match &self {
            &DBPool::None => {
                return Err(Error::from("un init DBPool!"));
            }
            #[cfg(feature = "mysql")]
            DBPool::Mysql(mysql, decoder) => {
                return Ok(DBPoolConn::Mysql(mysql.acquire().await?, decoder));
            }
            #[cfg(feature = "postgres")]
            DBPool::Postgres(pg, decoder) => {
                return Ok(DBPoolConn::Postgres(pg.acquire().await?, decoder));
            }
            #[cfg(feature = "sqlite")]
            DBPool::Sqlite(sqlite, decoder) => {
                return Ok(DBPoolConn::Sqlite(sqlite.acquire().await?, decoder));
            }
            #[cfg(feature = "mssql")]
            DBPool::Mssql(mssql, decoder) => {
                return Ok(DBPoolConn::Mssql(mssql.acquire().await?, decoder));
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    /// Attempts to retrieve a connection from the pool if there is one available.
    ///
    /// Returns `None` immediately if there are no idle connections available in the pool.
    pub fn try_acquire(&self) -> crate::Result<Option<DBPoolConn>> {
        match self {
            DBPool::None => {
                return Err(Error::from("un init DBPool!"));
            }
            #[cfg(feature = "mysql")]
            DBPool::Mysql(pool, decoder) => {
                let conn = pool.try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn::Mysql(conn.unwrap(), decoder)));
            }
            #[cfg(feature = "postgres")]
            DBPool::Postgres(pool, decoder) => {
                let conn = pool.try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn::Postgres(conn.unwrap(), decoder)));
            }
            #[cfg(feature = "sqlite")]
            DBPool::Sqlite(pool, decoder) => {
                let conn = pool.try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn::Sqlite(conn.unwrap(), decoder)));
            }
            #[cfg(feature = "mssql")]
            DBPool::Mssql(pool, decoder) => {
                let conn = pool.try_acquire();
                if conn.is_none() {
                    return Ok(None);
                }
                return Ok(Some(DBPoolConn::Mssql(conn.unwrap(), decoder)));
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn begin(&self) -> crate::Result<DBTx<'_>> {
        let mut tx = DBTx {
            driver_type: self.driver_type(),
            conn: Some(self.acquire().await?),
            done: true,
        };
        tx.begin().await?;
        Ok(tx)
    }

    pub async fn close(&self) {
        match self {
            DBPool::None => {
                return;
            }
            #[cfg(feature = "mysql")]
            DBPool::Mysql(pool, _) => {
                pool.close().await;
            }
            #[cfg(feature = "postgres")]
            DBPool::Postgres(pool, _) => {
                pool.close().await;
            }
            #[cfg(feature = "sqlite")]
            DBPool::Sqlite(pool, _) => {
                pool.close().await;
            }
            #[cfg(feature = "mssql")]
            DBPool::Mssql(pool, _) => {
                pool.close().await;
            }
            _ => {
                return;
            }
        }
    }
}

impl DriverType{
    pub fn make_db_query<'f, 's>(&self, sql: &'s str) -> crate::Result<DBQuery<'s>> {
        match self {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            &DriverType::Mysql => {
                return Ok(DBQuery {
                    driver_type: DriverType::Mysql,
                    #[cfg(feature = "mysql")]
                    mysql: Some(query(sql)),
                    #[cfg(feature = "postgres")]
                    postgres: None,
                    #[cfg(feature = "sqlite")]
                    sqlite: None,
                    #[cfg(feature = "mssql")]
                    mssql: None,
                });
            }
            &DriverType::Postgres => {
                return Ok(DBQuery {
                    driver_type: DriverType::Postgres,
                    #[cfg(feature = "mysql")]
                    mysql: None,
                    #[cfg(feature = "postgres")]
                    postgres: Some(query(sql)),
                    #[cfg(feature = "sqlite")]
                    sqlite: None,
                    #[cfg(feature = "mssql")]
                    mssql: None,
                });
            }
            &DriverType::Sqlite => {
                return Ok(DBQuery {
                    driver_type: DriverType::Sqlite,
                    #[cfg(feature = "mysql")]
                    mysql: None,
                    #[cfg(feature = "postgres")]
                    postgres: None,
                    #[cfg(feature = "sqlite")]
                    sqlite: Some(query(sql)),
                    #[cfg(feature = "mssql")]
                    mssql: None,
                });
            }
            &DriverType::Mssql => {
                return Ok(DBQuery {
                    driver_type: DriverType::Mssql,
                    #[cfg(feature = "mysql")]
                    mysql: None,
                    #[cfg(feature = "postgres")]
                    postgres: None,
                    #[cfg(feature = "sqlite")]
                    sqlite: None,
                    #[cfg(feature = "mssql")]
                    mssql: Some(query(sql)),
                });
            }
        }
    }

}

/// DBConnectOption all of support Database Options abstract struct.
/// use from(url:&str) or use from_mysql(),from_pg().... or other method init this.
#[derive(Debug, Clone)]
pub struct DBConnectOption {
    pub driver_type: DriverType,
    #[cfg(feature = "mysql")]
    pub mysql: Option<MySqlConnectOptions>,
    #[cfg(feature = "postgres")]
    pub postgres: Option<PgConnectOptions>,
    #[cfg(feature = "sqlite")]
    pub sqlite: Option<SqliteConnectOptions>,
    #[cfg(feature = "mssql")]
    pub mssql: Option<MssqlConnectOptions>,
}

impl DBConnectOption {
    #[cfg(feature = "mysql")]
    pub fn from_mysql(conn_opt: &MySqlConnectOptions) -> Result<Self> {
        let mut conn_opt = conn_opt.clone();
        conn_opt.log_slow_statements(log::LevelFilter::Off, Duration::from_secs(0));
        conn_opt.log_statements(log::LevelFilter::Off);
        return Ok(DBConnectOption {
            driver_type: DriverType::Mysql,
            #[cfg(feature = "mysql")]
            mysql: Some(conn_opt),
            #[cfg(feature = "postgres")]
            postgres: None,
            #[cfg(feature = "sqlite")]
            sqlite: None,
            #[cfg(feature = "mssql")]
            mssql: None,
        });
    }
    #[cfg(feature = "postgres")]
    pub fn from_pg(conn_opt: &PgConnectOptions) -> Result<Self> {
        let mut conn_opt = conn_opt.clone();
        conn_opt.log_slow_statements(log::LevelFilter::Off, Duration::from_secs(0));
        conn_opt.log_statements(log::LevelFilter::Off);
        return Ok(Self {
            driver_type: DriverType::Postgres,
            #[cfg(feature = "mysql")]
            mysql: None,
            #[cfg(feature = "postgres")]
            postgres: Some(conn_opt),
            #[cfg(feature = "sqlite")]
            sqlite: None,
            #[cfg(feature = "mssql")]
            mssql: None,
        });
    }

    #[cfg(feature = "sqlite")]
    pub fn from_sqlite(conn_opt: &SqliteConnectOptions) -> Result<Self> {
        let mut conn_opt = conn_opt.clone();
        conn_opt.log_slow_statements(log::LevelFilter::Off, Duration::from_secs(0));
        conn_opt.log_statements(log::LevelFilter::Off);
        return Ok(Self {
            driver_type: DriverType::Sqlite,
            #[cfg(feature = "mysql")]
            mysql: None,
            #[cfg(feature = "postgres")]
            postgres: None,
            #[cfg(feature = "sqlite")]
            sqlite: Some(conn_opt),
            #[cfg(feature = "mssql")]
            mssql: None,
        });
    }

    #[cfg(feature = "mssql")]
    pub fn from_mssql(conn_opt: &MssqlConnectOptions) -> Result<Self> {
        let mut conn_opt = conn_opt.clone();
        conn_opt.log_slow_statements(log::LevelFilter::Off, Duration::from_secs(0));
        conn_opt.log_statements(log::LevelFilter::Off);
        return Ok(Self {
            driver_type: DriverType::Mssql,
            #[cfg(feature = "mysql")]
            mysql: None,
            #[cfg(feature = "postgres")]
            postgres: None,
            #[cfg(feature = "sqlite")]
            sqlite: None,
            #[cfg(feature = "mssql")]
            mssql: Some(conn_opt),
        });
    }

    pub fn from(driver: &str) -> Result<Self> {
        if driver.starts_with("mysql") {
            #[cfg(feature = "mysql")]
                {
                    let mut conn_opt = MySqlConnectOptions::from_str(driver)?;
                    if !driver.contains("ssl-mode") {
                        conn_opt = conn_opt.ssl_mode(MySqlSslMode::Disabled);
                    }
                    return Self::from_mysql(&conn_opt);
                }
            #[cfg(not(feature = "mysql"))]
                {
                    return Err(Error::from("[rbatis] not enable feature!"));
                }
        } else if driver.starts_with("postgres") {
            #[cfg(feature = "postgres")]
                {
                    let mut conn_opt = PgConnectOptions::from_str(driver)?;
                    if !driver.contains("ssl-mode") && !driver.contains("sslmode") {
                        conn_opt = conn_opt.ssl_mode(PgSslMode::Disable);
                    }
                    return Self::from_pg(&conn_opt);
                }
            #[cfg(not(feature = "postgres"))]
                {
                    return Err(Error::from("[rbatis] not enable feature!"));
                }
        } else if driver.starts_with("sqlite") {
            #[cfg(feature = "sqlite")]
                {
                    let conn_opt = SqliteConnectOptions::from_str(driver)?;
                    return Self::from_sqlite(&conn_opt);
                }
            #[cfg(not(feature = "sqlite"))]
                {
                    return Err(Error::from("[rbatis] not enable feature!"));
                }
        } else if driver.starts_with("mssql") || driver.starts_with("sqlserver") {
            #[cfg(feature = "mssql")]
                {
                    let conn_opt = MssqlConnectOptions::from_str(driver)?;
                    return Self::from_mssql(&conn_opt);
                }
            #[cfg(not(feature = "mssql"))]
                {
                    return Err(Error::from("[rbatis] not enable feature!"));
                }
        } else {
            return Err(Error::from("unsupport driver type!"));
        }
    }
}


pub struct DBQuery<'q> {
    pub driver_type: DriverType,
    #[cfg(feature = "mysql")]
    pub mysql: Option<Query<'q, MySql, MySqlArguments>>,
    #[cfg(feature = "postgres")]
    pub postgres: Option<Query<'q, Postgres, PgArguments>>,
    #[cfg(feature = "sqlite")]
    pub sqlite: Option<Query<'q, Sqlite, SqliteArguments<'q>>>,
    #[cfg(feature = "mssql")]
    pub mssql: Option<Query<'q, Mssql, MssqlArguments>>,
}


impl<'q> DBQuery<'q> {
    pub fn bind_value(&mut self, t: Bson) -> crate::Result<()> {
        match &self.driver_type {
            &DriverType::None => {
                return Err(Error::from("un init DBPool!"));
            }
            #[cfg(feature = "mysql")]
            &DriverType::Mysql => {
                let mut q = self.mysql.take().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?;
                q = crate::db::bind_mysql::bind(t, q)?;
                self.mysql = Some(q);
            }
            #[cfg(feature = "postgres")]
            &DriverType::Postgres => {
                let mut q = self.postgres.take().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?;
                q = crate::db::bind_pg::bind(t, q)?;
                self.postgres = Some(q);
            }
            #[cfg(feature = "sqlite")]
            &DriverType::Sqlite => {
                let mut q = self.sqlite.take().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?;
                q = crate::db::bind_sqlite::bind(t, q)?;
                self.sqlite = Some(q);
            }
            #[cfg(feature = "mssql")]
            &DriverType::Mssql => {
                let mut q = self.mssql.take().ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?;
                q = crate::db::bind_mssql::bind(t, q)?;
                self.mssql = Some(q);
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
        return Ok(());
    }
}

#[derive(Debug)]
pub enum DBPoolConn<'a> {
    #[cfg(feature = "mysql")]
    Mysql(PoolConnection<MySql>, &'a Box<dyn DataDecoder>),
    #[cfg(feature = "postgres")]
    Postgres(PoolConnection<Postgres>, &'a Box<dyn DataDecoder>),
    #[cfg(feature = "sqlite")]
    Sqlite(PoolConnection<Sqlite>, &'a Box<dyn DataDecoder>),
    #[cfg(feature = "mssql")]
    Mssql(PoolConnection<Mssql>, &'a Box<dyn DataDecoder>),
}

impl<'a> DBPoolConn<'a> {
    pub fn driver_type(&self) -> DriverType {
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(_, _) => { DriverType::Mysql }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(_, _) => { DriverType::Postgres }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(_, _) => { DriverType::Sqlite }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(_, _) => { DriverType::Mssql }
        }
    }

    pub fn make_query<'f, 's>(&'f self, sql: &'s str) -> crate::Result<DBQuery<'s>> {
        return self.driver_type().make_db_query( sql);
    }

    pub fn check_alive(&self) -> crate::Result<()> {
        return Ok(());
    }

    pub async fn fetch<'q, T>(&mut self, sql: &'q str) -> crate::Result<(T, usize)>
        where
            T: DeserializeOwned,
    {
        self.check_alive()?;
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(conn, decoder) => {
                let async_stream: Vec<MySqlRow> = conn.fetch_all(sql).await?;
                let data = async_stream.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(conn, decoder) => {
                let async_stream: Vec<PgRow> = conn.fetch_all(sql).await?;
                let data = async_stream.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(conn, decoder) => {
                let data: Vec<SqliteRow> = conn.fetch_all(sql).await?;
                let data = data.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(conn, decoder) => {
                let async_stream: Vec<MssqlRow> = conn.fetch_all(sql).await?;
                let data = async_stream.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn exec_sql(&mut self, sql: &str) -> crate::Result<DBExecResult> {
        self.check_alive()?;
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(conn, _) => {
                let data: MySqlQueryResult = conn.execute(sql).await?;
                return Ok(DBExecResult::from(data));
            }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(conn, _) => {
                let data: PgQueryResult = conn.execute(sql).await?;
                return Ok(DBExecResult::from(data));
            }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(conn, _) => {
                let data: SqliteQueryResult = conn.execute(sql).await?;
                return Ok(DBExecResult::from(data));
            }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(conn, _) => {
                let data: MssqlQueryResult = conn.execute(sql).await?;
                return Ok(DBExecResult::from(data));
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn fetch_parperd<T>(&mut self, sql: DBQuery<'_>) -> crate::Result<(T, usize)>
        where
            T: DeserializeOwned,
    {
        self.check_alive()?;
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(conn, decoder) => {
                let data: Vec<MySqlRow> = conn
                    .fetch_all(sql.mysql.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                let data = data.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(conn, decoder) => {
                let data: Vec<PgRow> = conn
                    .fetch_all(sql.postgres.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                let data = data.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(conn, decoder) => {
                let data: Vec<SqliteRow> = conn
                    .fetch_all(sql.sqlite.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                let data = data.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(conn, decoder) => {
                let data: Vec<MssqlRow> = conn
                    .fetch_all(sql.mssql.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                let data = data.try_to_bson(decoder.as_ref())?.as_array().ok_or_else(|| Error::from("[rbatis-core] try_to_json is not array!"))?.to_owned();
                let return_len = data.len();
                let result = decode::<T>(data)?;
                Ok((result, return_len))
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<DBExecResult> {
        self.check_alive()?;
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(conn, _) => {
                let result: MySqlQueryResult = conn
                    .execute(sql.mysql.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                return Ok(DBExecResult::from(result));
            }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(conn, _) => {
                let data: PgQueryResult = conn
                    .execute(sql.postgres.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                return Ok(DBExecResult::from(data));
            }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(conn, _) => {
                let data: SqliteQueryResult = conn
                    .execute(sql.sqlite.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                return Ok(DBExecResult::from(data));
            }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(conn, _) => {
                let data: MssqlQueryResult = conn
                    .execute(sql.mssql.ok_or_else(|| Error::from("[rbatis-core] conn is none!"))?)
                    .await?;
                return Ok(DBExecResult::from(data));
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn begin(self) -> crate::Result<DBTx<'a>> {
        self.check_alive()?;
        let mut tx = DBTx {
            driver_type: self.driver_type(),
            conn: Some(self),
            done: true,
        };
        tx.begin().await;
        return Ok(tx);
    }

    pub async fn ping(&mut self) -> crate::Result<()> {
        self.check_alive()?;
        match self {
            #[cfg(feature = "mysql")]
            DBPoolConn::Mysql(conn, _) => {
                return Ok(conn.ping().await?);
            }
            #[cfg(feature = "postgres")]
            DBPoolConn::Postgres(conn, _) => {
                return Ok(conn.ping().await?);
            }
            #[cfg(feature = "sqlite")]
            DBPoolConn::Sqlite(conn, _) => {
                return Ok(conn.ping().await?);
            }
            #[cfg(feature = "mssql")]
            DBPoolConn::Mssql(conn, _) => {
                return Ok(conn.ping().await?);
            }
            _ => {
                return Err(Error::from("[rbatis] feature not enable!"));
            }
        }
    }

    pub async fn close(self) -> crate::Result<()> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct DBTx<'a> {
    pub driver_type: DriverType,
    pub conn: Option<DBPoolConn<'a>>,
    /// is tx done?
    pub done: bool,
}

impl<'a> DBTx<'a> {
    pub fn make_query<'f, 's>(&'f self, sql: &'s str) -> crate::Result<DBQuery<'s>> {
        return self.driver_type.make_db_query(sql);
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn take_conn(mut self) -> Option<DBPoolConn<'a>> {
        self.conn
    }

    pub fn get_conn_mut(&mut self) -> crate::Result<&mut DBPoolConn<'a>> {
        self.conn.as_mut().ok_or_else(|| Error::from("[rbatis-core] DBTx conn is none!"))
    }

    pub async fn begin(&mut self) -> crate::Result<()> {
        if !self.done {
            return Ok(());
        }
        let conn = self.get_conn_mut()?;
        conn.exec_sql("BEGIN").await?;
        self.done = false;
        return Ok(());
    }

    pub async fn commit(&mut self) -> crate::Result<()> {
        let conn = self.get_conn_mut()?;
        conn.exec_sql("COMMIT").await?;
        self.done = true;
        return Ok(());
    }

    pub async fn rollback(&mut self) -> crate::Result<()> {
        let conn = self.get_conn_mut()?;
        conn.exec_sql("ROLLBACK").await?;
        self.done = true;
        return Ok(());
    }

    pub async fn fetch<'q, T>(&mut self, sql: &'q str) -> crate::Result<(T, usize)>
        where
            T: DeserializeOwned,
    {
        let conn = self.get_conn_mut()?;
        return conn.fetch(sql).await;
    }

    pub async fn fetch_parperd<'q, T>(&mut self, sql: DBQuery<'q>) -> crate::Result<(T, usize)>
        where
            T: DeserializeOwned,
    {
        let conn = self.get_conn_mut()?;
        return conn.fetch_parperd(sql).await;
    }

    pub async fn exec_sql(&mut self, sql: &str) -> crate::Result<DBExecResult> {
        let conn = self.get_conn_mut()?;
        return conn.exec_sql(sql).await;
    }

    pub async fn exec_prepare(&mut self, sql: DBQuery<'_>) -> crate::Result<DBExecResult> {
        let conn = self.get_conn_mut()?;
        return conn.exec_prepare(sql).await;
    }
}

//databse db value
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBValue {
    pub type_info: Bson,
    pub data: Option<rbson::Binary>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DBExecResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

#[cfg(feature = "mysql")]
impl From<MySqlQueryResult> for DBExecResult {
    fn from(arg: MySqlQueryResult) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: Some(arg.last_insert_id() as i64),
        }
    }
}

#[cfg(feature = "postgres")]
impl From<PgQueryResult> for DBExecResult {
    fn from(arg: PgQueryResult) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: None,
        }
    }
}

#[cfg(feature = "sqlite")]
impl From<SqliteQueryResult> for DBExecResult {
    fn from(arg: SqliteQueryResult) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: Some(arg.last_insert_rowid()),
        }
    }
}

#[cfg(feature = "mssql")]
impl From<MssqlQueryResult> for DBExecResult {
    fn from(arg: MssqlQueryResult) -> Self {
        Self {
            rows_affected: arg.rows_affected(),
            last_insert_id: None,
        }
    }
}
