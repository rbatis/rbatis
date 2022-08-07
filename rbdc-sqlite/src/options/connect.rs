use crate::query::SqliteQuery;
use crate::type_info::Type;
use crate::{SqliteConnectOptions, SqliteConnection, SqliteRow, SqliteQueryResult};
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{StreamExt, TryStreamExt};
use log::LevelFilter;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::error::Error;
use rbs::Value;
use std::fmt::Write;
use std::time::Duration;

impl SqliteConnectOptions {
    pub fn connect(&self) -> BoxFuture<'_, Result<SqliteConnection, Error>> {
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
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            let many = {
                if params.len() == 0 {
                    self.fetch_many(SqliteQuery {
                        statement: Either::Left(sql),
                        arguments: params,
                        persistent: false,
                    })
                }else{
                    let stmt = self.prepare_with(&sql, &[]).await?;
                    self.fetch_many(SqliteQuery {
                        statement: Either::Right(stmt),
                        arguments: params,
                        persistent: true,
                    })
                }
            };
            let f:BoxStream<Result<SqliteRow,Error>>=many.try_filter_map(|step| async move {
                Ok(match step {
                    Either::Left(_) => None,
                    Either::Right(row) => Some(row),
                })
            }).boxed();
            let c:BoxFuture<Result<Vec<SqliteRow>,Error>>=f.try_collect().boxed();
            let v=c.await?;
            let mut data: Vec<Box<dyn Row>> = Vec::with_capacity(v.len());
            for x in v {
                data.push(Box::new(x));
            }
            Ok(data)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            let many = {
                if params.len() == 0 {
                    self.fetch_many(SqliteQuery {
                        statement: Either::Left(sql),
                        arguments: params,
                        persistent: false,
                    })
                }else{
                    let mut type_info = Vec::with_capacity(params.len());
                    for x in &params {
                        type_info.push(x.type_info());
                    }
                    let stmt = self.prepare_with(&sql, &type_info).await?;
                    self.fetch_many(SqliteQuery {
                        statement: Either::Right(stmt),
                        arguments: params,
                        persistent: true,
                    })
                }
            };
            let v: BoxStream<Result<SqliteQueryResult, Error>> = many.try_filter_map(|step| async move {
                Ok(match step {
                    Either::Left(rows) => Some(rows),
                    Either::Right(_) => None,
                })
            })
                .boxed();
            let v: SqliteQueryResult = v.try_collect().boxed().await?;
            return Ok(ExecResult {
                rows_affected: v.rows_affected(),
                last_insert_id: Value::U64(l.last_insert_rowid as u64),
            });
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async  { self.do_close().await })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move {
            self.worker
                .oneshot_cmd(|tx| crate::connection::Command::Ping { tx })
                .await?;
            Ok(())
        })
    }
}
