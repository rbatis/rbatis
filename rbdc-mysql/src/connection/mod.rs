use crate::protocol::statement::StmtClose;
use crate::protocol::text::{Ping, Quit};
use crate::stmt::MySqlStatementMetadata;
use either::Either;
use futures_core::future::BoxFuture;
use futures_util::{FutureExt, StreamExt};
use rbdc::common::StatementCache;
use rbdc::db::{Connection, Row};
use rbdc::Error;
use rbs::Value;
use std::fmt::{self, Debug, Formatter};

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
    pub stream: MySqlStream,
    // cache by query string to the statement id and metadata
    pub cache_statement: StatementCache<(u32, MySqlStatementMetadata)>,
}

impl Debug for MySqlConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MySqlConnection").finish()
    }
}

impl MySqlConnection {
    fn close(mut self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move {
            self.stream.send_packet(Quit).await?;
            self.stream.shutdown().await?;

            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
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
            if params.len() == 0 {
                let mut many = self.fetch_many(MysqlQuery {
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
                let mut many = self.fetch_many(MysqlQuery {
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

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            if params.len() == 0 {
                let mut many = self.fetch_many(MysqlQuery {
                    statement: Either::Left(sql),
                    arguments: params,
                    persistent: false,
                });
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {
                            return Ok(l.rows_affected);
                        }
                        Either::Right(r) => {}
                    }
                }
                return Ok(0);
            } else {
                let stmt = self.prepare_with(&sql, &[]).await?;
                let mut many = self.fetch_many(MysqlQuery {
                    statement: Either::Right(stmt),
                    arguments: params,
                    persistent: true,
                });
                while let Some(item) = many.next().await {
                    match item? {
                        Either::Left(l) => {
                            return Ok(l.rows_affected);
                        }
                        Either::Right(r) => {}
                    }
                }
                return Ok(0);
            }
        })
    }
}
