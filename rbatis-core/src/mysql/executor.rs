use futures_core::future::BoxFuture;

use crate::cursor::Cursor;
use crate::describe::{Column, Describe};
use crate::executor::{Execute, Executor, RefExecutor};
use crate::mysql::protocol::{
    self, ColumnDefinition, ComQuery, ComStmtExecute, ComStmtPrepare, ComStmtPrepareOk, FieldFlags,
    Status,
};
use crate::mysql::{MySql, MySqlArguments, MySqlCursor, MySqlTypeInfo};

impl super::MySqlConnection {
    // Creates a prepared statement for the passed query string
    async fn prepare(&mut self, query: &str) -> crate::Result<ComStmtPrepareOk> {
        // https://dev.mysql.com/doc/dev/mysql-server/8.0.11/page_protocol_com_stmt_prepare.html
        self.stream.send(ComStmtPrepare { query }, true).await?;

        // Should receive a COM_STMT_PREPARE_OK or ERR_PACKET
        let packet = self.stream.receive().await?;

        if packet[0] == 0xFF {
            return self.stream.handle_err();
        }

        ComStmtPrepareOk::read(packet)
    }

    async fn drop_column_defs(&mut self, count: usize) -> crate::Result<()> {
        for _ in 0..count {
            let _column = ColumnDefinition::read(self.stream.receive().await?)?;
        }

        if count > 0 {
            self.stream.maybe_receive_eof().await?;
        }

        Ok(())
    }

    // Gets a cached prepared statement ID _or_ prepares the statement if not in the cache
    // At the end we should have [cache_statement] and [cache_statement_columns] filled
    async fn get_or_prepare(&mut self, query: &str) -> crate::Result<u32> {
        if let Some(&id) = self.cache_statement.get(query) {
            Ok(id)
        } else {
            let stmt = self.prepare(query).await?;

            self.cache_statement.insert(query.into(), stmt.statement_id);

            // COM_STMT_PREPARE returns the input columns
            // We make no use of that data, so cycle through and drop them
            self.drop_column_defs(stmt.params as usize).await?;

            // COM_STMT_PREPARE next returns the output columns
            // We just drop these as we get these when we execute the query
            self.drop_column_defs(stmt.columns as usize).await?;

            Ok(stmt.statement_id)
        }
    }

    pub(crate) async fn run(
        &mut self,
        query: &str,
        arguments: Option<MySqlArguments>,
    ) -> crate::Result<Option<u32>> {
        self.stream.wait_until_ready().await?;
        self.stream.is_ready = false;

        if let Some(arguments) = arguments {
            let statement_id = self.get_or_prepare(query).await?;

            // https://dev.mysql.com/doc/dev/mysql-server/8.0.11/page_protocol_com_stmt_execute.html
            self.stream
                .send(
                    ComStmtExecute {
                        cursor: protocol::Cursor::NO_CURSOR,
                        statement_id,
                        params: &arguments.params,
                        null_bitmap: &arguments.null_bitmap,
                        param_types: &arguments.param_types,
                    },
                    true,
                )
                .await?;

            Ok(Some(statement_id))
        } else {
            // https://dev.mysql.com/doc/dev/mysql-server/8.0.11/page_protocol_com_query.html
            self.stream.send(ComQuery { query }, true).await?;

            Ok(None)
        }
    }

    async fn affected_rows(&mut self) -> crate::Result<u64> {
        let mut rows = 0;

        loop {
            let id = self.stream.receive().await?[0];

            match id {
                0x00 | 0xFE if self.stream.packet().len() < 0xFF_FF_FF => {
                    // ResultSet row can begin with 0xfe byte (when using text protocol
                    // with a field length > 0xffffff)

                    let status = if let Some(eof) = self.stream.maybe_handle_eof()? {
                        eof.status
                    } else {
                        let ok = self.stream.handle_ok()?;

                        rows += ok.affected_rows;
                        ok.status
                    };

                    if !status.contains(Status::SERVER_MORE_RESULTS_EXISTS) {
                        self.is_ready = true;
                        break;
                    }
                }

                0xFF => {
                    return self.stream.handle_err();
                }

                _ => {}
            }
        }

        Ok(rows)
    }

    // method is not named describe to work around an intellijrust bug
    // otherwise it marks someone trying to describe the connection as "method is private"
    async fn do_describe(&mut self, query: &str) -> crate::Result<Describe<MySql>> {
        self.stream.wait_until_ready().await?;

        let stmt = self.prepare(query).await?;

        let mut param_types = Vec::with_capacity(stmt.params as usize);
        let mut result_columns = Vec::with_capacity(stmt.columns as usize);

        for _ in 0..stmt.params {
            let param = ColumnDefinition::read(self.stream.receive().await?)?;
            param_types.push(MySqlTypeInfo::from_column_def(&param));
        }

        if stmt.params > 0 {
            self.stream.maybe_receive_eof().await?;
        }

        for _ in 0..stmt.columns {
            let column = ColumnDefinition::read(self.stream.receive().await?)?;

            result_columns.push(Column::<MySql> {
                type_info: MySqlTypeInfo::from_column_def(&column),
                name: column.column_alias.or(column.column),
                table_id: column.table_alias.or(column.table),
                // TODO(@abonander): Should this be None in some cases?
                non_null: Some(column.flags.contains(FieldFlags::NOT_NULL)),
            });
        }

        if stmt.columns > 0 {
            self.stream.maybe_receive_eof().await?;
        }

        Ok(Describe {
            param_types: param_types.into_boxed_slice(),
            result_columns: result_columns.into_boxed_slice(),
        })
    }
}

impl Executor for super::MySqlConnection {
    type Database = MySql;

    fn execute<'e, 'q: 'e, 'c: 'e, E: 'e>(
        &'c mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<u64>>
    where
        E: Execute<'q, Self::Database>,
    {
        Box::pin(async move {
            let (query, arguments) = query.into_parts();

            self.run(query, arguments).await?;
            self.affected_rows().await
        })
    }

    fn cursor<'q, E>(&mut self, query: E) -> MySqlCursor<'_, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        MySqlCursor::from_connection(self, query)
    }

    #[doc(hidden)]
    fn describe<'e, 'q, E: 'e>(
        &'e mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>>
    where
        E: Execute<'q, Self::Database>,
    {
        Box::pin(async move { self.do_describe(query.into_parts().0).await })
    }
}

impl<'c> RefExecutor<'c> for &'c mut super::MySqlConnection {
    type Database = MySql;

    fn fetch_by_ref<'q, E>(self, query: E) -> MySqlCursor<'c, 'q>
    where
        E: Execute<'q, Self::Database>,
    {
        MySqlCursor::from_connection(self, query)
    }
}
