use crate::connection::MySqlConnection;
use crate::options::MySqlConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, Driver};
use rbdc::Error;
use std::pin::Pin;
use std::str::FromStr;

pub struct MysqlDriver {}

impl Driver for MysqlDriver {
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let url = url.to_owned();
        Box::pin(async move {
            let conn = MySqlConnection::establish(&MySqlConnectOptions::from_str(&url)?).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }
}
