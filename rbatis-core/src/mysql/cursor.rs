use std::collections::HashMap;
use std::sync::Arc;

use futures_core::future::BoxFuture;
use serde::de::DeserializeOwned;

use crate::connection::ConnectionSource;
use crate::cursor::Cursor;
use crate::executor::Execute;
use crate::mysql::{MySql, MySqlArguments, MySqlConnection, MySqlRow, MySqlTypeInfo};
use crate::mysql::protocol::{ColumnCount, ColumnDefinition, Row, Status};
use crate::pool::Pool;
use crate::decode::decode_result;

pub struct MySqlCursor<'c, 'q> {
    source: ConnectionSource<'c, MySqlConnection>,
    query: Option<(&'q str, Option<MySqlArguments>)>,
    column_names: Arc<HashMap<Box<str>, u16>>,
    column_types: Vec<MySqlTypeInfo>,
    binary: bool,
}

impl crate::cursor::private::Sealed for MySqlCursor<'_, '_> {}

impl<'c, 'q> Cursor<'c, 'q> for MySqlCursor<'c, 'q> {
    type Database = MySql;

    #[doc(hidden)]
    fn from_pool<E>(pool: &Pool<MySqlConnection>, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, MySql>,
    {
        Self {
            source: ConnectionSource::Pool(pool.clone()),
            column_names: Arc::default(),
            column_types: Vec::new(),
            binary: true,
            query: Some(query.into_parts()),
        }
    }

    #[doc(hidden)]
    fn from_connection<E>(conn: &'c mut MySqlConnection, query: E) -> Self
        where
            Self: Sized,
            E: Execute<'q, MySql>,
    {
        Self {
            source: ConnectionSource::ConnectionRef(conn),
            column_names: Arc::default(),
            column_types: Vec::new(),
            binary: true,
            query: Some(query.into_parts()),
        }
    }

    fn next(&mut self) -> BoxFuture<crate::Result<Option<MySqlRow<'_>>>> {
        Box::pin(next(self))
    }

    fn decode<T>(&mut self) -> BoxFuture<Result<T, crate::Error>>
        where T: DeserializeOwned {
        Box::pin(async move {
            let mut arr = vec![];
            while let Some(row) = self.next().await? as Option<MySqlRow<'_>> {
                let mut m = serde_json::Map::new();
                let keys = row.names.keys();
                for x in keys {
                    let key = x.to_string();
                    let v: serde_json::Value = row.json_decode_impl(key.as_str()).unwrap();
                    m.insert(key, v);
                }
                arr.push(serde_json::Value::Object(m));
            }
            let r = decode_result(arr)?;
            return Ok(r);
        })
    }
}


async fn next<'a, 'c: 'a, 'q: 'a>(
    cursor: &'a mut MySqlCursor<'c, 'q>,
) -> crate::Result<Option<MySqlRow<'a>>> {
    let mut conn = cursor.source.resolve().await?;

    // The first time [next] is called we need to actually execute our
    // contained query. We guard against this happening on _all_ next calls
    // by using [Option::take] which replaces the potential value in the Option with `None
    let mut initial = if let Some((query, arguments)) = cursor.query.take() {
        let statement = conn.run(query, arguments).await?;

        // No statement ID = TEXT mode
        cursor.binary = statement.is_some();

        true
    } else {
        false
    };

    loop {
        let packet_id = conn.stream.receive().await?[0];

        match packet_id {
            // OK or EOF packet
            0x00 | 0xFE
            if conn.stream.packet().len() < 0xFF_FF_FF && (packet_id != 0x00 || initial) =>
                {
                    let status = if let Some(eof) = conn.stream.maybe_handle_eof()? {
                        eof.status
                    } else {
                        conn.stream.handle_ok()?.status
                    };

                    if status.contains(Status::SERVER_MORE_RESULTS_EXISTS) {
                        // There is more to this query
                        initial = true;
                    } else {
                        conn.is_ready = true;
                        return Ok(None);
                    }
                }

            // ERR packet
            0xFF => {
                conn.is_ready = true;
                return conn.stream.handle_err();
            }

            _ if initial => {
                // At the start of the results we expect to see a
                // COLUMN_COUNT followed by N COLUMN_DEF

                let cc = ColumnCount::read(conn.stream.packet())?;

                // We use these definitions to get the actual column types that is critical
                // in parsing the rows coming back soon

                cursor.column_types.clear();
                cursor.column_types.reserve(cc.columns as usize);

                let mut column_names = HashMap::with_capacity(cc.columns as usize);

                for i in 0..cc.columns {
                    let column = ColumnDefinition::read(conn.stream.receive().await?)?;

                    cursor
                        .column_types
                        .push(MySqlTypeInfo::from_nullable_column_def(&column));

                    if let Some(name) = column.name() {
                        column_names.insert(name.to_owned().into_boxed_str(), i as u16);
                    }
                }

                if cc.columns > 0 {
                    conn.stream.maybe_receive_eof().await?;
                }

                cursor.column_names = Arc::new(column_names);
                initial = false;
            }

            _ if !cursor.binary || packet_id == 0x00 => {
                let row = Row::read(
                    conn.stream.packet(),
                    &cursor.column_types,
                    &mut conn.current_row_values,
                    cursor.binary,
                )?;

                let row = MySqlRow {
                    row,
                    names: Arc::clone(&cursor.column_names),
                };

                return Ok(Some(row));
            }

            _ => {
                return conn.stream.handle_unexpected();
            }
        }
    }
}
