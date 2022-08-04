use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use crate::connection::PgConnection;
use crate::options::{PgConnectOptions, PgSslMode};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::error::Error;
use std::time::Duration;
use url::quirks::host;
use rbdc::net::CertificateInput;
use rbs::{from_value, Value};
use std::borrow::Cow;

impl ConnectOptions for PgConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let v = PgConnection::establish(self)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(v) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, arg: &str) -> Result<(), Error> {
        *self=PgConnectOptions::from_str(arg).map_err(|e| Error::from(e.to_string()))?;
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}
