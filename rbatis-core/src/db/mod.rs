#[cfg(feature = "postgres")]
pub mod bind_pg;
#[cfg(feature = "mysql")]
pub mod bind_mysql;
#[cfg(feature = "sqlite")]
pub mod bind_sqlite;
#[cfg(feature = "mssql")]
pub mod bind_mssql;


use std::time::Duration;
use rbson::Bson;

use chrono::NaiveDateTime;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub use db_adapter::{
    DBConnectOption, DBExecResult, DBPool, DBPoolConn, DBQuery, DBTx,
};
use crate::convert::StmtConvert;
use crate::db::db_adapter::DataDecoder;

pub mod db_adapter;


#[derive(Debug)]
pub struct DBPoolOptions {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub test_before_acquire: bool,
    pub decoder: Box<dyn DataDecoder>,
}

impl Default for DBPoolOptions {
    fn default() -> Self {
        Self {
            // pool a maximum of 10 connections to the same database
            max_connections: 10,
            // don't open connections until necessary
            min_connections: 0,
            // try to connect for 10 seconds before erroring
            connect_timeout: Duration::from_secs(60),
            // reap connections that have been alive > 30 minutes
            // prevents unbounded live-leaking of memory due to naive prepared statement caching
            // see src/cache.rs for context
            max_lifetime: Some(Duration::from_secs(1800)),
            // don't reap connections based on idle time
            idle_timeout: None,
            // If true, test the health of a connection on acquire
            test_before_acquire: true,
            decoder: Box::new(DefaultDecoder {}),
        }
    }
}

impl DBPoolOptions {
    pub fn new() -> Self {
        DBPoolOptions::default()
    }
}

#[derive(Clone, Debug)]
pub struct DefaultDecoder {}

impl DataDecoder for DefaultDecoder {
    fn decode(&self, _key: &str, _data: &mut Bson) -> crate::Result<()> {
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum DriverType {
    None = 0,
    Mysql = 1,
    Postgres = 2,
    Sqlite = 3,
    Mssql = 4,
}

impl DriverType {
    pub fn is_number_type(&self) -> bool {
        match self {
            DriverType::Postgres|DriverType::Mssql => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

