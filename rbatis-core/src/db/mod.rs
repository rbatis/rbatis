use std::time::Duration;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub use db_adapter::{DBConnection, DBConnectOption, DBExecResult, DBPool, DBPoolConn, DBQuery, DBTx};

pub mod db_adapter;

#[derive(Debug, Clone, Copy)]
pub struct DBPoolOptions {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub test_before_acquire: bool,
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
        }
    }
}

impl DBPoolOptions {
    pub fn new() -> Self {
        DBPoolOptions::default()
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
            DriverType::Postgres => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}