use crate::options::MySqlConnectOptions;
use percent_encoding::percent_decode_str;
use rbdc::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use url::{ParseError, Url};

impl FromStr for MySqlConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let url: Url = s
            .parse()
            .map_err(|e: ParseError| Error::from(e.to_string()))?;
        let mut options = Self::new();

        if let Some(host) = url.host_str() {
            options = options.host(host);
        }

        if let Some(port) = url.port() {
            options = options.port(port);
        }

        let username = url.username();
        if !username.is_empty() {
            options = options.username(&*percent_decode_str(username).decode_utf8()?);
        }

        if let Some(password) = url.password() {
            options = options.password(&*percent_decode_str(password).decode_utf8()?);
        }

        let path = url.path().trim_start_matches('/');
        if !path.is_empty() {
            options = options.database(path);
        }

        for (key, value) in url.query_pairs().into_iter() {
            match &*key {
                "ssl-mode" => {
                    options = options.ssl_mode(value.parse()?);
                }

                "ssl-ca" => {
                    options = options.ssl_ca(&*value);
                }

                "charset" => {
                    options = options.charset(&*value);
                }

                "collation" => {
                    options = options.collation(&*value);
                }

                "statement-cache-capacity" => {
                    options = options.statement_cache_capacity(
                        value
                            .parse()
                            .map_err(|e: ParseIntError| Error::from(e.to_string()))?,
                    );
                }

                "socket" => {
                    options = options.socket(&*value);
                }

                _ => {}
            }
        }

        Ok(options)
    }
}

#[test]
fn it_parses_username_with_at_sign_correctly() {
    let uri = "mysql://user@hostname:password@hostname:5432/database";
    let opts = MySqlConnectOptions::from_str(uri).unwrap();

    assert_eq!("user@hostname", &opts.username);
}

#[test]
fn it_parses_password_with_non_ascii_chars_correctly() {
    let uri = "mysql://username:p@ssw0rd@hostname:5432/database";
    let opts = MySqlConnectOptions::from_str(uri).unwrap();

    assert_eq!(Some("p@ssw0rd".into()), opts.password);
}
