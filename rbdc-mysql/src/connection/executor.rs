use super::MySqlStream;
use crate::connection::stream::Waiting;
use crate::connection::MySqlConnection;
use crate::io::MySqlBufExt;
use crate::protocol::response::Status;
use crate::protocol::statement::{
    BinaryRow, Execute as StatementExecute, Prepare, PrepareOk, StmtClose,
};
use crate::protocol::text::{ColumnDefinition, Query, TextRow};
use crate::query::MysqlQuery;
use crate::query_result::MySqlQueryResult;
use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use crate::row::MySqlRow;
use crate::stmt::{MySqlArguments, MySqlStatement, MySqlStatementMetadata};
use crate::value::MySqlValueFormat;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_core::Stream;
use futures_util::{pin_mut, TryStreamExt};
use rbdc::ext::ustr::UStr;
use rbdc::{try_stream, Error};
use std::collections::HashMap;
use std::sync::Arc;

impl MySqlConnection {
    async fn get_or_prepare<'c>(
        &mut self,
        sql: &str,
        persistent: bool,
    ) -> Result<(u32, MySqlStatementMetadata), Error> {
        if let Some(statement) = self.cache_statement.get_mut(sql) {
            // <MySqlStatementMetadata> is internally reference-counted
            return Ok((*statement).clone());
        }

        // https://dev.mysql.com/doc/internals/en/com-stmt-prepare.html
        // https://dev.mysql.com/doc/internals/en/com-stmt-prepare-response.html#packet-COM_STMT_PREPARE_OK

        self.stream.send_packet(Prepare { query: sql }).await?;

        let ok: PrepareOk = self.stream.recv().await?;

        // the parameter definitions are very unreliable so we skip over them
        // as we have little use

        if ok.params > 0 {
            for _ in 0..ok.params {
                let _def: ColumnDefinition = self.stream.recv().await?;
            }

            self.stream.maybe_recv_eof().await?;
        }

        // the column definitions are berefit the type information from the
        // to-be-bound parameters; we will receive the output column definitions
        // once more on execute so we wait for that

        let mut columns = Vec::with_capacity(ok.columns as usize);

        let column_names = if ok.columns > 0 {
            recv_result_metadata(&mut self.stream, ok.columns as usize, &mut columns).await?
        } else {
            Default::default()
        };

        let id = ok.statement_id;
        let metadata = MySqlStatementMetadata {
            parameters: ok.params as usize,
            columns: Arc::new(columns),
            column_names: Arc::new(column_names),
        };

        if persistent && self.cache_statement.is_enabled() {
            // in case of the cache being full, close the least recently used statement
            if let Some((id, _)) = self.cache_statement.insert(sql, (id, metadata.clone())) {
                self.stream.send_packet(StmtClose { statement: id }).await?;
            }
        }

        Ok((id, metadata))
    }

    #[allow(clippy::needless_lifetimes)]
    async fn run<'e, 'q: 'e>(
        &'e mut self,
        sql: &'q str,
        arguments: Option<MySqlArguments>,
        persistent: bool,
    ) -> Result<impl Stream<Item = Result<Either<MySqlQueryResult, MySqlRow>, Error>> + 'e, Error>
    {
        self.stream.wait_until_ready().await?;
        self.stream.waiting.push_back(Waiting::Result);

        Ok(Box::pin(try_stream! {
            // make a slot for the shared column data
            // as long as a reference to a row is not held past one iteration, this enables us
            // to re-use this memory freely between result sets
            let mut columns = Arc::new(Vec::with_capacity({
                if let Some(arguments) = &arguments {
                     arguments.len()
                }else{
                     0
                }
            }));

            let (mut column_names, format, mut needs_metadata) = if let Some(arguments) = arguments {
                let (id, metadata) = self.get_or_prepare(
                    sql,
                    persistent,
                )
                .await?;

                // https://dev.mysql.com/doc/internals/en/com-stmt-execute.html
                self.stream
                    .send_packet(StatementExecute {
                        statement_id: id,
                        arguments: &arguments,
                    })
                    .await?;

                (metadata.column_names, MySqlValueFormat::Binary, false)
            } else {
                // https://dev.mysql.com/doc/internals/en/com-query.html
                self.stream.send_packet(Query(sql)).await?;

                (Arc::default(), MySqlValueFormat::Text, true)
            };

            loop {
                // query response is a meta-packet which may be one of:
                //  Ok, Err, ResultSet, or (unhandled) LocalInfileRequest
                let mut packet = self.stream.recv_packet().await?;

                if packet[0] == 0x00 || packet[0] == 0xff {
                    // first packet in a query response is OK or ERR
                    // this indicates either a successful query with no rows at all or a failed query
                    let ok = packet.ok()?;

                    let rows_affected = ok.affected_rows;
                    let done = MySqlQueryResult {
                        rows_affected,
                        last_insert_id: ok.last_insert_id,
                    };

                    r#yield!(Either::Left(done));

                    if ok.status.contains(Status::SERVER_MORE_RESULTS_EXISTS) {
                        // more result sets exist, continue to the next one
                        continue;
                    }

                    self.stream.waiting.pop_front();
                    return Ok(());
                }

                // otherwise, this first packet is the start of the result-set metadata,
                *self.stream.waiting.front_mut().unwrap() = Waiting::Row;

                let num_columns = packet.get_uint_lenenc() as usize; // column count

                if needs_metadata {
                    column_names = Arc::new(recv_result_metadata(&mut self.stream, num_columns, Arc::make_mut(&mut columns)).await?);
                } else {
                    // next time we hit here, it'll be a new result set and we'll need the
                    // full metadata
                    needs_metadata = true;

                    recv_result_columns(&mut self.stream, num_columns, Arc::make_mut(&mut columns)).await?;
                }

                // finally, there will be none or many result-rows
                loop {
                    let packet = self.stream.recv_packet().await?;

                    if packet[0] == 0xfe && packet.len() < 9 {
                        let eof = packet.eof(self.stream.capabilities)?;

                        r#yield!(Either::Left(MySqlQueryResult {
                            rows_affected: 0,
                            last_insert_id: 0,
                        }));

                        if eof.status.contains(Status::SERVER_MORE_RESULTS_EXISTS) {
                            // more result sets exist, continue to the next one
                            *self.stream.waiting.front_mut().unwrap() = Waiting::Result;
                            break;
                        }

                        self.stream.waiting.pop_front();
                        return Ok(());
                    }

                    let row = match format {
                        MySqlValueFormat::Binary => packet.decode_with::<BinaryRow, _>(&columns)?.0,
                        MySqlValueFormat::Text => packet.decode_with::<TextRow, _>(&columns)?.0,
                    };

                    let v = Either::Right(MySqlRow {
                        row,
                        format,
                        columns: Arc::clone(&columns),
                        column_names: Arc::clone(&column_names),
                    });

                    r#yield!(v);
                }
            }
        }))
    }
}

impl MySqlConnection {
    pub async fn execute(&mut self, sql: &str) -> Result<Option<MySqlRow>, Error> {
        self.fetch_optional(MysqlQuery {
            statement: Either::Left(sql.to_string()),
            arguments: vec![],
            persistent: false,
        })
        .await
    }

    pub fn fetch_many(
        &mut self,
        query: MysqlQuery,
    ) -> BoxStream<'_, Result<Either<MySqlQueryResult, MySqlRow>, Error>> {
        let sql = query.sql().to_owned();
        let persistent = query.persistent();
        Box::pin(try_stream! {
            let arguments = query.take_arguments()?;
            let s = self.run(&sql, arguments, persistent).await?;
            pin_mut!(s);

            while let Some(v) = s.try_next().await? {
                r#yield!(v);
            }

            Ok(())
        })
    }

    pub fn fetch_optional(
        &mut self,
        query: MysqlQuery,
    ) -> BoxFuture<'_, Result<Option<MySqlRow>, Error>> {
        let mut s = self.fetch_many(query);
        Box::pin(async move {
            while let Some(v) = s.try_next().await? {
                if let Either::Right(r) = v {
                    return Ok(Some(r));
                }
            }

            Ok(None)
        })
    }

    pub fn prepare_with<'e>(
        &'e mut self,
        sql: &'e str,
        _: &'e [MySqlTypeInfo],
    ) -> BoxFuture<'e, Result<MySqlStatement, Error>> {
        let sql = sql.to_string();
        Box::pin(async move {
            self.stream.wait_until_ready().await?;

            let (_, metadata) = self.get_or_prepare(&sql, true).await?;

            Ok(MySqlStatement {
                sql: sql,
                // metadata has internal Arcs for expensive data structures
                metadata: metadata.clone(),
            })
        })
    }

    // #[doc(hidden)]
    // pub fn describe<'e, 'q: 'e>(mut self, sql: &'q str) -> BoxFuture<'e, Result<Describe, Error>> {
    //     Box::pin(async move {
    //         self.stream.wait_until_ready().await?;
    //
    //         let (_, metadata) = self.get_or_prepare(sql, false).await?;
    //
    //         let columns = (&*metadata.columns).clone();
    //
    //         let nullable = columns
    //             .iter()
    //             // .map(|col| {
    //             //     col.flags
    //             //         .map(|flags| !flags.contains(ColumnFlags::NOT_NULL))
    //             // })
    //             .collect();
    //
    //         Ok(Describe {
    //             parameters: Some(Either::Right(metadata.parameters)),
    //             columns,
    //             nullable,
    //         })
    //     })
    // }
}

async fn recv_result_columns(
    stream: &mut MySqlStream,
    num_columns: usize,
    columns: &mut Vec<MySqlColumn>,
) -> Result<(), Error> {
    columns.clear();
    columns.reserve(num_columns);

    for ordinal in 0..num_columns {
        columns.push(recv_next_result_column(&stream.recv().await?, ordinal)?);
    }

    if num_columns > 0 {
        stream.maybe_recv_eof().await?;
    }

    Ok(())
}

fn recv_next_result_column(def: &ColumnDefinition, ordinal: usize) -> Result<MySqlColumn, Error> {
    // if the alias is empty, use the alias
    // only then use the name
    let name = match (def.name()?, def.alias()?) {
        (_, alias) if !alias.is_empty() => UStr::new(alias),
        (name, _) => UStr::new(name),
    };

    let type_info = MySqlTypeInfo::from_column(&def);

    Ok(MySqlColumn {
        name,
        type_info,
        ordinal,
        // flags: Some(def.flags),
    })
}

async fn recv_result_metadata(
    stream: &mut MySqlStream,
    num_columns: usize,
    columns: &mut Vec<MySqlColumn>,
) -> Result<HashMap<UStr, (usize, MySqlTypeInfo)>, Error> {
    // the result-set metadata is primarily a listing of each output
    // column in the result-set

    let mut column_names = HashMap::with_capacity(num_columns);

    columns.clear();
    columns.reserve(num_columns);

    for ordinal in 0..num_columns {
        let def: ColumnDefinition = stream.recv().await?;

        let column = recv_next_result_column(&def, ordinal)?;

        column_names.insert(column.name.clone(), (ordinal, column.type_info.clone()));
        columns.push(column);
    }

    stream.maybe_recv_eof().await?;

    Ok(column_names)
}
