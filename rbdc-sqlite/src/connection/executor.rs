use crate::query::SqliteQuery;
use crate::{ SqliteConnection, SqliteQueryResult, SqliteRow, SqliteStatement, SqliteTypeInfo};
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{TryFutureExt, TryStreamExt};
use rbdc::error::Error;

impl SqliteConnection {
    pub fn fetch_many(
        &mut self,
        query: SqliteQuery,
    ) -> BoxStream<'_, Result<Either<SqliteQueryResult, SqliteRow>, Error>> {
        let sql = query.sql().to_string();
        let persistent = query.persistent() && !query.arguments.is_empty();
        let arguments = query.take_arguments();
        Box::pin(
            self.worker
                .execute(sql, arguments, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .try_flatten_stream(),
        )
    }

    pub fn fetch_optional(
        &mut self,
        query: SqliteQuery,
    ) -> BoxFuture<'_, Result<Option<SqliteRow>, Error>> {
        let sql = query.sql().to_owned();
        let persistent = query.persistent() && !query.arguments.is_empty();
        let arguments = query.take_arguments();
        Box::pin(async move {
            let stream = self
                .worker
                .execute(sql, arguments, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .try_flatten_stream();

            futures_util::pin_mut!(stream);

            while let Some(res) = stream.try_next().await? {
                if let Either::Right(row) = res {
                    return Ok(Some(row));
                }
            }

            Ok(None)
        })
    }

    pub fn prepare_with<'a>(
        &'a mut self,
        sql: &'a str,
        _parameters: &[SqliteTypeInfo],
    ) -> BoxFuture<'_, Result<SqliteStatement, Error>> {
        Box::pin(async move {
            let statement = self.worker.prepare(sql).await?;

            Ok(SqliteStatement {
                sql: sql.into(),
                ..statement
            })
        })
    }
}
