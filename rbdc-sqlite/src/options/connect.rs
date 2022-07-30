use rbdc::error::Error;
use crate::{SqliteConnectOptions, SqliteConnection};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use std::fmt::Write;
use std::time::Duration;
use either::Either;
use futures_core::stream::BoxStream;
use futures_util::TryStreamExt;
use rbdc::db::{Connection, Row};
use rbs::Value;
use crate::query::SqliteQuery;

impl SqliteConnectOptions {
    pub fn connect(&self) -> BoxFuture<'_, Result<SqliteConnection, Error>>
    {
        Box::pin(async move {
            let mut conn = SqliteConnection::establish(self).await?;

            // send an initial sql statement comprised of options
            let mut init = String::new();

            // This is a special case for sqlcipher. When the `key` pragma
            // is set, we have to make sure it's executed first in order.
            if let Some(pragma_key_password) = self.pragmas.get("key") {
                write!(init, "PRAGMA key = {}; ", pragma_key_password).ok();
            }

            for (key, value) in &self.pragmas {
                // Since we've already written the possible `key` pragma
                // above, we shall skip it now.
                if key == "key" {
                    continue;
                }
                write!(init, "PRAGMA {} = {}; ", key, value).ok();
            }

            conn.exec(&*init, vec![]).await?;

            if !self.collations.is_empty() {
                let mut locked = conn.lock_handle().await?;

                for collation in &self.collations {
                    collation.create(&mut locked.guard.handle)?;
                }
            }

            Ok(conn)
        })
    }
}

impl Connection for SqliteConnection {
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        todo!()
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, Error>> {
        todo!()
    }

    fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
        todo!()
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        todo!()
    }
}
