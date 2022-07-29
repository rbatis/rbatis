#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use crate::HashMap;

use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use rbdc::db::{MetaData, Row};
use rbdc::Error;
use rbs::Value;
use crate::statement::StatementHandle;
use crate::{Sqlite, SqliteColumn, SqliteValue, SqliteValueRef};

/// Implementation of [`Row`] for SQLite.
pub struct SqliteRow {
    pub(crate) values: Box<[SqliteValue]>,
    pub(crate) columns: Arc<Vec<SqliteColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl crate::row::private_row::Sealed for SqliteRow {}

// Accessing values from the statement object is
// safe across threads as long as we don't call [sqlite3_step]

// we block ourselves from doing that by only exposing
// a set interface on [StatementHandle]

unsafe impl Send for SqliteRow {}
unsafe impl Sync for SqliteRow {}

impl SqliteRow {
    pub(crate) fn current(
        statement: &StatementHandle,
        columns: &Arc<Vec<SqliteColumn>>,
        column_names: &Arc<HashMap<UStr, usize>>,
    ) -> Self {
        let size = statement.column_count();
        let mut values = Vec::with_capacity(size);

        for i in 0..size {
            values.push(unsafe {
                let raw = statement.column_value(i);

                SqliteValue::new(raw, columns[i].type_info.clone())
            });
        }

        Self {
            values: values.into_boxed_slice(),
            columns: Arc::clone(columns),
            column_names: Arc::clone(column_names),
        }
    }
}

impl Row for SqliteRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        todo!()
    }

    fn get(&mut self, i: usize) -> Option<Value> {
        todo!()
    }
}
impl SqliteRow{
    fn columns(&self) -> &[SqliteColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<SqliteValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(SqliteValueRef::value(&self.values[index]))
    }
}

impl ColumnIndex<SqliteRow> for &'_ str {
    fn index(&self, row: &SqliteRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::from(format!("ColumnNotFound{}",self)))
            .map(|v| *v)
    }
}
