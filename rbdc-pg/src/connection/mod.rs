use crate::message::{
    Close, Message, MessageFormat, Query, ReadyForQuery, Terminate, TransactionStatus,
};
use crate::options::PgConnectOptions;
use crate::query::PgQuery;
use crate::statement::PgStatementMetadata;
use crate::type_info::PgTypeInfo;
use crate::types::encode::{Encode, TypeInfo};
use crate::types::Oid;
use either::Either;
use futures_core::future::BoxFuture;
use futures_util::{FutureExt, StreamExt, TryFutureExt};
use rbdc::common::StatementCache;
use rbdc::db::{Connection, Row};
use rbdc::ext::ustr::UStr;
use rbdc::io::Decode;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;

pub use self::stream::PgStream;

pub(crate) mod describe;
mod establish;
mod executor;
mod sasl;
mod stream;
mod tls;

/// A connection to a PostgreSQL database.
pub struct PgConnection {
    // underlying TCP or UDS stream,
    // wrapped in a potentially TLS stream,
    // wrapped in a buffered stream
    pub(crate) stream: PgStream,

    // process id of this backend
    // used to send cancel requests
    #[allow(dead_code)]
    process_id: u32,

    // secret key of this backend
    // used to send cancel requests
    #[allow(dead_code)]
    secret_key: u32,

    // sequence of statement IDs for use in preparing statements
    // in PostgreSQL, the statement is prepared to a user-supplied identifier
    next_statement_id: Oid,

    // cache statement by query string to the id and columns
    cache_statement: StatementCache<(Oid, Arc<PgStatementMetadata>)>,

    // cache user-defined types by id <-> info
    cache_type_info: HashMap<Oid, PgTypeInfo>,
    cache_type_oid: HashMap<UStr, Oid>,

    // number of ReadyForQuery messages that we are currently expecting
    pub(crate) pending_ready_for_query_count: usize,

    // current transaction status
    transaction_status: TransactionStatus,
    pub(crate) transaction_depth: usize,
}

impl PgConnection {
    /// the version number of the server in `libpq` format
    pub fn server_version_num(&self) -> Option<u32> {
        self.stream.server_version_num
    }

    // will return when the connection is ready for another query
    pub async fn wait_until_ready(&mut self) -> Result<(), Error> {
        if !self.stream.wbuf.is_empty() {
            self.stream.flush().await?;
        }

        while self.pending_ready_for_query_count > 0 {
            let message = self.stream.recv().await?;

            if let MessageFormat::ReadyForQuery = message.format {
                self.handle_ready_for_query(message)?;
            }
        }

        Ok(())
    }

    async fn recv_ready_for_query(&mut self) -> Result<(), Error> {
        let r: ReadyForQuery = self
            .stream
            .recv_expect(MessageFormat::ReadyForQuery)
            .await?;

        self.pending_ready_for_query_count -= 1;
        self.transaction_status = r.transaction_status;

        Ok(())
    }

    fn handle_ready_for_query(&mut self, message: Message) -> Result<(), Error> {
        self.pending_ready_for_query_count -= 1;
        self.transaction_status = ReadyForQuery::decode(message.contents)?.transaction_status;

        Ok(())
    }

    /// Queue a simple query (not prepared) to execute the next time this connection is used.
    ///
    /// Used for rolling back transactions and releasing advisory locks.
    pub(crate) fn queue_simple_query(&mut self, query: &str) {
        self.pending_ready_for_query_count += 1;
        self.stream.write(Query(query));
    }
}

impl Debug for PgConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PgConnection").finish()
    }
}
impl PgConnection {
    fn cached_statements_size(&self) -> usize {
        self.cache_statement.len()
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(async move {
            let mut cleared = 0_usize;

            self.wait_until_ready().await?;

            while let Some((id, _)) = self.cache_statement.remove_lru() {
                self.stream.write(Close::Statement(id));
                cleared += 1;
            }

            if cleared > 0 {
                self.write_sync();
                self.stream.flush().await?;

                self.wait_for_close_complete(cleared).await?;
                self.recv_ready_for_query().await?;
            }

            Ok(())
        })
    }
    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        !self.stream.wbuf.is_empty()
    }
    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        self.wait_until_ready().boxed()
    }
}
impl PgConnection {
    fn close(mut self) -> BoxFuture<'static, Result<(), Error>> {
        // The normal, graceful termination procedure is that the frontend sends a Terminate
        // message and immediately closes the connection.

        // On receipt of this message, the backend closes the
        // connection and terminates.
        Box::pin(async move {
            self.stream.send(Terminate).await?;
            self.stream.shutdown().await?;

            Ok(())
        })
    }
}

impl Connection for PgConnection {
    fn close(&mut self) -> BoxFuture<'static, Result<(), Error>> {
        let c = self.close();
        Box::pin(async move { c.await })
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        // By sending a comment we avoid an error if the connection was in the middle of a rowset
        self.exec("/* RBDC ping */", vec![]).map_ok(|_| ()).boxed()
    }

    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        let sql = sql.to_owned();
        Box::pin(async move {
            if params.len() == 0 {
                let mut many = self.fetch_many(PgQuery {
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
                let stmt = self.prepare_with(sql, &[]).await?;
                let mut many = self.fetch_many(PgQuery {
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
                let mut many = self.fetch_many(PgQuery {
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
                let mut type_info = Vec::with_capacity(params.len());
                for x in &params {
                    type_info.push(x.type_info());
                }
                let stmt = self.prepare_with(sql, &type_info).await?;
                let mut many = self.fetch_many(PgQuery {
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
