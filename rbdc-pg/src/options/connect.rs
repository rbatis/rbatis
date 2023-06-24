use crate::connection::PgConnection;
use crate::options::PgConnectOptions;
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::error::Error;
use std::any::Any;
use std::str::FromStr;

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
        *self = PgConnectOptions::from_str(arg).map_err(|e| Error::from(e.to_string()))?;
        Ok(())
    }
}
