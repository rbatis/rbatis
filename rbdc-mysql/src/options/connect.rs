use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;
use crate::connection::MySqlConnection;
use crate::options::{MySqlConnectOptions, MySqlSslMode};
use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::Error;
use rbdc::net::CertificateInput;
use rbs::{from_value, Value};

impl ConnectOptions for MySqlConnectOptions {
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let mut conn = MySqlConnection::establish(self).await?;

            // After the connection is established, we initialize by configuring a few
            // connection parameters

            // https://mariadb.com/kb/en/sql-mode/

            // PIPES_AS_CONCAT - Allows using the pipe character (ASCII 124) as string concatenation operator.
            //                   This means that "A" || "B" can be used in place of CONCAT("A", "B").

            // NO_ENGINE_SUBSTITUTION - If not set, if the available storage engine specified by a CREATE TABLE is
            //                          not available, a warning is given and the default storage
            //                          engine is used instead.

            // NO_ZERO_DATE - Don't allow '0000-00-00'. This is invalid in Rust.

            // NO_ZERO_IN_DATE - Don't allow 'YYYY-00-00'. This is invalid in Rust.

            // --

            // Setting the time zone allows us to assume that the output
            // from a TIMESTAMP field is UTC

            // --

            // https://mathiasbynens.be/notes/mysql-utf8mb4

            let mut options = String::new();
            // options.push_str(r#"SET sql_mode=(SELECT CONCAT(@@sql_mode, ',PIPES_AS_CONCAT,NO_ENGINE_SUBSTITUTION')),"#);
            // options.push_str(r#"time_zone='+00:00',"#);
            options.push_str(&format!(
                r#"SET NAMES {} COLLATE {};"#,
                conn.stream.charset.as_str(),
                conn.stream.collation.as_str()
            ));

            conn.execute(&*options).await?;

            let r: Box<dyn Connection> = Box::new(conn);
            Ok(r)
        })
    }

    fn set(&mut self, arg: HashMap<&str, Value>) {
        for (k, v) in arg {
            match k {
                "host" => {
                    self.host = v.as_str().unwrap_or_default().to_string();
                }
                "port" => {
                    self.port = (v.as_u64().unwrap_or_default() as u16);
                }
                "socket" => {
                    self.socket = Some(v.as_str().unwrap_or_default().parse().unwrap());
                }
                "username" => {
                    self.username = v.as_str().unwrap_or_default().to_string();
                }
                "password" => {
                    self.password = Some(v.as_str().unwrap_or_default().to_string());
                }
                "database" => {
                    self.database = Some(v.as_str().unwrap_or_default().to_string());
                }
                "ssl_mode" => {
                    match from_value::<MySqlSslMode>(v) {
                        Ok(v) => {
                            self.ssl_mode = v;
                        }
                        Err(_) => {}
                    }
                }
                "ssl_ca" => {
                    let v = from_value::<Option<CertificateInput>>(v);
                    match v {
                        Ok(v) => {
                            self.ssl_ca = v;
                        }
                        Err(_) => {}
                    }
                }
                "statement_cache_capacity" => {
                    match from_value::<usize>(v) {
                        Ok(v) => {
                            self.statement_cache_capacity = v;
                        }
                        Err(_) => {}
                    }
                }
                "charset" => match from_value::<String>(v) {
                    Ok(v) => {
                        self.charset = v;
                    }
                    Err(_) => {}
                },
                "collation" => match from_value::<Option<String>>(v) {
                    Ok(v) => {
                        self.collation = v;
                    }
                    Err(_) => {}
                },
                _ => {}
            }
        }
    }

    fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
        *self=MySqlConnectOptions::from_str(uri).map_err(|e|Error::from(e.to_string()))?;
        Ok(())
    }

    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
        self
    }
}
