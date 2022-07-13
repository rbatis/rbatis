use crate::protocol;
use crate::result_set::MySqlColumn;
use crate::value::{MySqlValueFormat, MySqlValueRef};
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub(crate) row: protocol::Row,
    pub(crate) format: MySqlValueFormat,
    pub(crate) columns: Arc<Vec<MySqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

pub trait Index {
    fn columns(&self) -> &[MySqlColumn];

    fn try_get_raw(&self, index: usize) -> Result<MySqlValueRef<'_>, Error>;
}

impl Index for MySqlRow {
    fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    fn try_get_raw(&self, index: usize) -> Result<MySqlValueRef<'_>, Error> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.get(index as usize);

        Ok(MySqlValueRef {
            format: self.format,
            row: Some(&self.row.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}
