use crate::protocol;
use crate::protocol::text::ColumnType;
use crate::result_set::MySqlColumn;
use crate::value::{MySqlValueFormat, MySqlValueRef};
use rbdc::db::{MetaData, Row};
use rbdc::error::Error;
use rbdc::ext::ustr::UStr;
use rbs::Value;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub row: protocol::Row,
    pub format: MySqlValueFormat,
    pub columns: Arc<Vec<MySqlColumn>>,
    pub column_names: Arc<HashMap<UStr, usize>>,
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

impl MetaData for MySqlRow {
    fn column_len(&self) -> usize {
        self.column_names.len()
    }

    fn column_name(&self, i: usize) -> String {
        for (s, idx) in self.column_names.deref() {
            if idx.eq(&i) {
                return s.to_string();
            }
        }
        return String::new();
    }

    fn column_type(&self, i: usize) -> String {
        match self.columns.get(i) {
            None => String::new(),
            Some(v) => format!("{:?}", v.type_info.r#type),
        }
    }
}

impl Row for MySqlRow {
    fn meta_data(&self) -> &dyn MetaData {
        self
    }

    fn get(&self, i: usize) -> Option<Value> {
        match self.columns.get(i) {
            None => None,
            Some(v) => Some(Value::from(v)),
        }
    }
}

impl From<&MySqlColumn> for Value {
    fn from(v: &MySqlColumn) -> Self {
        match v.type_info.r#type {
            ColumnType::Decimal => {}
            ColumnType::Tiny => {}
            ColumnType::Short => {}
            ColumnType::Long => {}
            ColumnType::Float => {}
            ColumnType::Double => {}
            ColumnType::Null => {}
            ColumnType::Timestamp => {}
            ColumnType::LongLong => {}
            ColumnType::Int24 => {}
            ColumnType::Date => {}
            ColumnType::Time => {}
            ColumnType::Datetime => {}
            ColumnType::Year => {}
            ColumnType::VarChar => {}
            ColumnType::Bit => {}
            ColumnType::Json => {}
            ColumnType::NewDecimal => {}
            ColumnType::Enum => {}
            ColumnType::Set => {}
            ColumnType::TinyBlob => {}
            ColumnType::MediumBlob => {}
            ColumnType::LongBlob => {}
            ColumnType::Blob => {}
            ColumnType::VarString => {}
            ColumnType::String => {}
            ColumnType::Geometry => {}
        }
        todo!()
    }
}
