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

    fn set(&mut self, arg: HashMap<&str, Value>) {
        for (k, v) in arg {
            match k {
                "host" => {
                    if let Ok(v) = from_value::<String>(v) {
                        self.host = v;
                    }
                }
                "port" => {
                    if let Ok(v) = from_value::<u16>(v) {
                        self.port = v;
                    }
                }
                "socket" => {
                    if let Ok(v) = from_value::<Option<PathBuf>>(v) {
                        self.socket = v;
                    }
                }
                "username" => {
                    if let Ok(v) = from_value::<String>(v) {
                        self.username = v;
                    }
                }
                "password" => {
                    if let Ok(v) = from_value::<Option<String>>(v) {
                        self.password = v;
                    }
                }
                "database" => {
                    if let Ok(v) = from_value::<Option<String>>(v) {
                        self.database = v;
                    }
                }
                "ssl_mode" => {
                    if let Ok(v) = from_value::<PgSslMode>(v) {
                        self.ssl_mode = v;
                    }
                }
                "ssl_root_cert" => {
                    if let Ok(v) = from_value::<Option<CertificateInput>>(v) {
                        self.ssl_root_cert = v;
                    }
                }
                "statement_cache_capacity" => {
                    if let Ok(v) = from_value::<usize>(v) {
                        self.statement_cache_capacity = v;
                    }
                }
                "application_name" => {
                    if let Ok(v) = from_value::<Option<String>>(v) {
                        self.application_name = v;
                    }
                }
                "extra_float_digits" => {
                    if let Ok(v) = from_value::<Option<Cow<'static, str>>>(v) {
                        self.extra_float_digits = v;
                    }
                }
                "options" => {
                    {
                        if let Ok(v) = from_value::<Option<String>>(v) {
                            self.options = v;
                        }
                    }
                }
                &_ => {}
            }
        }
    }

    fn set_uri(&mut self, arg: &str) -> Result<(), Error> {
        PgConnectOptions::from_str(arg).map_err(|e| Error::from(e.to_string()))?;
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}
