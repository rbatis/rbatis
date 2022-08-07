use crate::protocol::statement::StmtClose;
use crate::protocol::text::{Ping, Quit};
use crate::stmt::MySqlStatementMetadata;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{FutureExt, StreamExt, TryStreamExt};
use rbdc::common::StatementCache;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::Error;
use rbs::Value;
use std::fmt::{self, Debug, Formatter};
use std::future::join;
use std::ops::{Deref, DerefMut};

mod auth;
mod establish;
mod executor;
mod stream;
mod tls;

use crate::query::MysqlQuery;
use crate::query_result::MySqlQueryResult;
use crate::row::MySqlRow;
pub(crate) use stream::MySqlStream;

const MAX_PACKET_SIZE: u32 = 1024;

/// A connection to a MySQL database.
pub struct MySqlConnection {
    // underlying TCP stream,
    // wrapped in a potentially TLS stream,
    // wrapped in a buffered stream
    pub stream: DropBox<MySqlStream>,
    // cache by query string to the statement id and metadata
    pub cache_statement: StatementCache<(u32, MySqlStatementMetadata)>,
}

impl Debug for MySqlConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MySqlConnection").finish()
    }
}

pub struct DropBox<T> {
    pub inner: Option<T>,
}

impl<T> Deref for DropBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().expect("conn closed")
    }
}

impl<T> DerefMut for DropBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().expect("conn closed")
    }
}

impl MySqlConnection {
    #[inline]
    async fn do_close(&mut self) -> Result<(), Error> {
        self.stream.send_packet(Quit).await?;
        self.stream.shutdown().await?;
        Ok(())
    }

    fn do_ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            self.stream.wait_until_ready().await?;
            self.stream.send_packet(Ping).await?;
            self.stream.recv_ok().await?;

            Ok(())
        })
    }

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        self.stream.wait_until_ready().boxed()
    }

    fn cached_statements_size(&self) -> usize {
        self.cache_statement.len()
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            while let Some((statement_id, _)) = self.cache_statement.remove_lru() {
                self.stream
                    .send_packet(StmtClose {
                        statement: statement_id,
                    })
                    .await?;
            }

            Ok(())
        })
    }

    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        !self.stream.wbuf.is_empty()
    }
}

impl Connection for MySqlConnection {
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            let many = {
                if params.len() == 0 {
                    self.fetch_many(MysqlQuery {
                        statement: Either::Left(sql),
                        arguments: params,
                        persistent: false,
                    })
                } else {
                    let stmt = self.prepare_with(&sql, &[]).await?;
                    self.fetch_many(MysqlQuery {
                        statement: Either::Right(stmt),
                        arguments: params,
                        persistent: true,
                    })
                }
            };
            let f: BoxStream<Result<MySqlRow, Error>> = many
                .try_filter_map(|step| async move {
                    Ok(match step {
                        Either::Left(_) => None,
                        Either::Right(row) => Some(row),
                    })
                })
                .boxed();
            let c: BoxFuture<Result<Vec<MySqlRow>, Error>> = f.try_collect().boxed();
            let v = c.await?;
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
                    self.fetch_many(MysqlQuery {
                        statement: Either::Left(sql),
                        arguments: params,
                        persistent: false,
                    })
                } else {
                    let stmt = self.prepare_with(&sql, &[]).await?;
                    self.fetch_many(MysqlQuery {
                        statement: Either::Right(stmt),
                        arguments: params,
                        persistent: true,
                    })
                }
            };
            let v: BoxStream<Result<MySqlQueryResult, Error>> = many
                .try_filter_map(|step| async move {
                    Ok(match step {
                        Either::Left(rows) => Some(rows),
                        Either::Right(_) => None,
                    })
                })
                .boxed();
            let v: MySqlQueryResult = v.try_collect().boxed().await?;
            return Ok(ExecResult {
                rows_affected: v.rows_affected,
                last_insert_id: v.last_insert_id.into(),
            });
        })
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        let c = self.do_close();
        Box::pin(async { c.await })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        let c = self.do_ping();
        Box::pin(async move { c.await })
    }
}

impl Drop for MySqlConnection {
    fn drop(&mut self) {
        let stream = self.stream.inner.take();
        rbdc::rt::spawn(async move {
            match stream {
                None => {}
                Some(mut s) => {
                    s.send_packet(Quit).await;
                    s.shutdown().await;
                }
            }
        });
    }
}
