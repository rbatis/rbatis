use futures_core::future::BoxFuture;
use serde::de::DeserializeOwned;

use crate::connection::ConnectionSource;
use crate::cursor::Cursor;
use crate::decode::json_decode;
use crate::executor::Execute;
use crate::pool::Pool;
use crate::sqlite::{Sqlite, SqliteArguments, SqliteConnection, SqliteRow};
use crate::sqlite::statement::Step;

pub struct SqliteCursor<'c, 'q> {
    pub(super) source: ConnectionSource<'c, SqliteConnection>,
    query: &'q str,
    arguments: Option<SqliteArguments>,
    pub(super) statement: Option<Option<usize>>,
}

impl crate::cursor::private::Sealed for SqliteCursor<'_, '_> {}

impl<'c, 'q> Cursor<'c, 'q> for SqliteCursor<'c, 'q> {
    type Database = Sqlite;

    #[doc(hidden)]
    fn from_pool<E>(pool: &Pool<SqliteConnection>, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, Sqlite>,
    {
        let (query, arguments) = query.into_parts();

        Self {
            source: ConnectionSource::Pool(pool.clone()),
            statement: None,
            query,
            arguments,
        }
    }

    #[doc(hidden)]
    fn from_connection<E>(conn: &'c mut SqliteConnection, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, Sqlite>,
    {
        let (query, arguments) = query.into_parts();

        Self {
            source: ConnectionSource::ConnectionRef(conn),
            statement: None,
            query,
            arguments,
        }
    }

    fn next(&mut self) -> BoxFuture<crate::Result<Option<SqliteRow<'_>>>> {
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
            while let Some(row) = self.next().await? as Option<SqliteRow<'_>> {
                let mut m = serde_json::Map::new();
                for key in &row.column_keys {
                    let v: serde_json::Value = row.json_decode_impl(key.as_str())?;
                    m.insert(key.clone(), v);
                }
                arr.push(serde_json::Value::Object(m));
            }
            return Ok(arr);
        })
    }
}


async fn next<'a, 'c: 'a, 'q: 'a>(
    cursor: &'a mut SqliteCursor<'c, 'q>,
) -> crate::Result<Option<SqliteRow<'a>>> {
    let conn = cursor.source.resolve().await?;

    loop {
        if cursor.statement.is_none() {
            let key = conn.prepare(&mut cursor.query, cursor.arguments.is_some())?;

            if let Some(arguments) = &mut cursor.arguments {
                conn.statement_mut(key).bind(arguments)?;
            }

            cursor.statement = Some(key);
        }

        let key = cursor.statement.unwrap();
        let statement = conn.statement_mut(key);

        let step = statement.step().await?;

        match step {
            Step::Row => {
                let column_size = statement.data_count();
                let mut column_keys = Vec::with_capacity(column_size);
                for index in 0..column_size {
                    column_keys.push(statement.column_name(index).to_string());
                }
                return Ok(Some(SqliteRow {
                    column_keys,
                    statement: key,
                    connection: conn,
                }));
            }

            Step::Done if cursor.query.is_empty() => {
                return Ok(None);
            }

            Step::Done => {
                cursor.statement = None;
                // continue
            }
        }
    }
}
