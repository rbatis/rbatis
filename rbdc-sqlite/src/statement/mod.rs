
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use crate::{Sqlite, SqliteArguments, SqliteColumn, SqliteTypeInfo};
use either::Either;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

mod handle;
mod r#virtual;

pub(crate) use handle::StatementHandle;
pub(crate) use r#virtual::VirtualStatement;

#[derive(Debug, Clone)]
#[allow(clippy::rc_buffer)]
pub struct SqliteStatement {
    pub(crate) sql: String,
    pub(crate) parameters: usize,
    pub(crate) columns: Arc<Vec<SqliteColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl  SqliteStatement {
   pub fn to_owned(&self) -> SqliteStatement {
        SqliteStatement {
            sql: self.sql.clone(),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn parameters(&self) -> Option<Either<&[SqliteTypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    pub fn columns(&self) -> &[SqliteColumn] {
        &self.columns
    }
}