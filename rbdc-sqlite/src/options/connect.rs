use rbdc::error::Error;
use crate::{SqliteConnectOptions, SqliteConnection};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use std::fmt::Write;
use std::time::Duration;
use either::Either;
use futures_core::stream::BoxStream;
use futures_util::{StreamExt, TryStreamExt};
use rbdc::db::{Connection, ExecResult, Row};
use rbs::Value;
use crate::query::SqliteQuery;
use crate::type_info::Type;

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
        let sql = sql.to_owned();
        Box::pin(async move {
            if params.len() == 0 {
                let mut many = self.fetch_many(SqliteQuery {
                    statement: Either::Left(sql),
                    arguments: params,
                    persistent: false,
                });
                let mut data: Vec<Box<dyn Row>> = Vec::new();
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {}
                        Either::Right(r) => {
                            data.push(Box::new(r));
                        }
                    }
                }
                return Ok(data);
            } else {
                let stmt = self.prepare_with(&sql, &[]).await?;
                let mut many = self.fetch_many(SqliteQuery {
                    statement: Either::Right(stmt),
                    arguments: params,
                    persistent: true,
                });
                let mut data: Vec<Box<dyn Row>> = Vec::new();
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {}
                        Either::Right(r) => {
                            data.push(Box::new(r));
                        }
                    }
                }
                return Ok(data);
            }
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            if params.len() == 0 {
                let mut many = self.fetch_many(SqliteQuery {
                    statement: Either::Left(sql),
                    arguments: params,
                    persistent: false,
                });
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {
                            return Ok(ExecResult {
                                rows_affected: l.rows_affected(),
                                last_insert_id: Value::U64(l.last_insert_rowid as u64)
                            });
                        }
                        Either::Right(r) => {}
                    }
                }
                return Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::Null
                });
            } else {
                let mut type_info = Vec::with_capacity(params.len());
                for x in &params {
                    type_info.push(x.type_info());
                }
                let stmt = self.prepare_with(&sql, &type_info).await?;
                let mut many = self.fetch_many(SqliteQuery {
                    statement: Either::Right(stmt),
                    arguments: params,
                    persistent: true,
                });
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {
                            return Ok(ExecResult {
                                rows_affected: l.rows_affected(),
                                last_insert_id: Value::U64(l.last_insert_rowid as u64)
                            });
                        }
                        Either::Right(r) => {}
                    }
                }
                return Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::Null
                });
            }
        })
    }

    fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
        let c = self.close();
        Box::pin(async move { c.await })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move{
            self.worker.oneshot_cmd(|tx| crate::connection::Command::Ping { tx }).await?;
            Ok(())
        })
    }
}
