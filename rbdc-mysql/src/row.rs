use crate::meta_data::MysqlMetaData;
use crate::protocol;
use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use crate::value::{MySqlValue, MySqlValueFormat, MySqlValueRef};
use rbdc::db::{MetaData, Row};
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub row: protocol::Row,
    pub format: MySqlValueFormat,
    pub columns: Arc<Vec<MySqlColumn>>,
    pub column_names: Arc<HashMap<UStr, (usize, MySqlTypeInfo)>>,
}

impl MySqlRow {
    pub fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    pub fn try_get(&self, index: usize) -> Result<MySqlValueRef<'_>, Error> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.get(index as usize);
        Ok(MySqlValueRef {
            format: self.format,
            type_info: column.type_info.clone(),
            value,
        })
    }

    pub fn try_take(&mut self, index: usize) -> Option<MySqlValue> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.take(index)?;
        Some(MySqlValue {
            value: Some(value),
            type_info: column.type_info.clone(),
            format: self.format,
        })
    }
}

impl Row for MySqlRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(MysqlMetaData {
            inner: self.column_names.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Result<Value,Error> {
        match self.try_take(i) {
            None => Ok(Value::Null),
            Some(v) => Ok(Value::from(v)),
        }
    }
}
