#![allow(clippy::rc_buffer)]

use crate::decode::Decode;
use crate::statement::StatementHandle;
use crate::{SqliteColumn, SqliteValue, SqliteValueRef};
use rbdc::db::{MetaData, Row};
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use rbs::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Implementation of [`Row`] for SQLite.
#[derive(Debug)]
pub struct SqliteRow {
    pub(crate) values: Vec<SqliteValue>,
    pub(crate) columns: Arc<Vec<SqliteColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

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
            values: values,
            columns: Arc::clone(columns),
            column_names: Arc::clone(column_names),
        }
    }
}

#[derive(Debug)]
pub struct SqliteMetaData {
    pub columns: Arc<Vec<SqliteColumn>>,
}

impl MetaData for SqliteMetaData {
    fn column_len(&self) -> usize {
        self.columns.len()
    }

    fn column_name(&self, i: usize) -> String {
        self.columns[i].name.to_string()
    }

    fn column_type(&self, i: usize) -> String {
        self.columns[i].type_info.to_string()
    }
}

impl Row for SqliteRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(SqliteMetaData {
            columns: self.columns.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Result<Value, Error> {
        match self.try_take(i) {
            Err(e) => Err(Error::from(format!("get error index:{},error:{}", i, e))),
            Ok(v) => Value::decode(v),
        }
    }
}

impl SqliteRow {
    fn columns(&self) -> &[SqliteColumn] {
        &self.columns
    }

    fn try_get_raw(&self, index: usize) -> Result<SqliteValueRef<'_>, Error> {
        Ok(SqliteValueRef::value(&self.values[index]))
    }

    fn try_take(&mut self, index: usize) -> Result<SqliteValue, Error> {
        if (index + 1) > self.values.len() {
            return Err(Error::from("try_take out of range!"));
        }
        Ok(self.values.remove(index))
    }
}
