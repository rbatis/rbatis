use futures_core::future::BoxFuture;
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::connection::ConnectionSource;
use crate::cursor::Cursor;
use crate::decode::json_decode;
use crate::executor::Execute;
use crate::pool::Pool;
use crate::postgres::{PgArguments, PgConnection, PgRow, Postgres};
use crate::postgres::protocol::{DataRow, Message, ReadyForQuery, RowDescription};
use crate::postgres::row::Statement;

pub struct PgCursor<'c, 'q> {
    source: ConnectionSource<'c, PgConnection>,
    query: Option<(&'q str, Option<PgArguments>)>,
    statement: Arc<Statement>,
}

impl crate::cursor::private::Sealed for PgCursor<'_, '_> {}

impl<'c, 'q> Cursor<'c, 'q> for PgCursor<'c, 'q> {
    type Database = Postgres;

    #[doc(hidden)]
    fn from_pool<E>(pool: &Pool<PgConnection>, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, Postgres>,
    {
        Self {
            source: ConnectionSource::Pool(pool.clone()),
            statement: Arc::default(),
            query: Some(query.into_parts()),
        }
    }

    #[doc(hidden)]
    fn from_connection<E>(conn: &'c mut PgConnection, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, Postgres>,
    {
        Self {
            source: ConnectionSource::ConnectionRef(conn),
            statement: Arc::default(),
            query: Some(query.into_parts()),
        }
    }

    fn next(&mut self) -> BoxFuture<crate::Result<Option<PgRow<'_>>>> {
        Box::pin(next(self))
    }

    fn decode_json<T>(&mut self) -> BoxFuture<Result<T, crate::Error>>
        where T: DeserializeOwned {
        Box::pin(async move {
            let arr = self.fetch_json().await?;
            let r = json_decode(arr)?;
            return Ok(r);
        })
    }

    fn fetch_json(&mut self) -> BoxFuture<'_, Result<Vec<serde_json::Value>, crate::Error>> {
        Box::pin(async move {
            let mut arr = vec![];
            while let Some(row) = self.next().await? as Option<PgRow<'_>> {
                let mut m = serde_json::Map::new();
                let keys = row.statement.names.keys();
                for x in keys {
                    let key = x.to_string();
                    let v: serde_json::Value = row.json_decode_impl(key.as_str())?;
                    m.insert(key, v);
                }
                arr.push(serde_json::Value::Object(m));
            }
            return Ok(arr);
        })
    }
}


async fn next<'a, 'c: 'a, 'q: 'a>(
    cursor: &'a mut PgCursor<'c, 'q>,
) -> crate::Result<Option<PgRow<'a>>> {
    let mut conn = cursor.source.resolve().await?;

    // The first time [next] is called we need to actually execute our
    // contained query. We guard against this happening on _all_ next calls
    // by using [Option::take] which replaces the potential value in the Option with `None
    if let Some((query, arguments)) = cursor.query.take() {
        let statement = conn.run(query, arguments).await?;

        // If there is a statement ID, this is a non-simple or prepared query
        if let Some(statement) = statement {
            // A prepared statement will re-use the previous column map
            cursor.statement = Arc::clone(&conn.cache_statement[&statement]);
        }

        // A non-prepared query must be described each time
        // We wait until we hit a RowDescription
    }

    loop {
        match conn.stream.receive().await? {
            // Indicates that a phase of the extended query flow has completed
            // We as rbatis_core don't generally care as long as it is happening
            Message::ParseComplete | Message::BindComplete => {}

            // Indicates that _a_ query has finished executing
            Message::CommandComplete => {}

            // Indicates that all queries have finished executing
            Message::ReadyForQuery => {
                // TODO: How should we handle an ERROR status form ReadyForQuery
                let _ready = ReadyForQuery::read(conn.stream.buffer())?;

                conn.is_ready = true;
                break;
            }

            Message::RowDescription => {
                // NOTE: This is only encountered for unprepared statements
                let rd = RowDescription::read(conn.stream.buffer())?;
                cursor.statement = Arc::new(
                    conn.parse_row_description(rd, Default::default(), None, false)
                        .await?,
                );
            }

            Message::DataRow => {
                let data = DataRow::read(conn.stream.buffer(), &mut conn.current_row_values)?;

                return Ok(Some(PgRow {
                    statement: Arc::clone(&cursor.statement),
                    data,
                }));
            }

            message => {
                return Err(protocol_err!("next: unexpected message: {:?}", message).into());
            }
        }
    }

    Ok(None)
}
