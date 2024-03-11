use crate::column::PgColumn;
use crate::message::DataRow;
use crate::meta_data::PgMetaData;
use crate::statement::PgStatementMetadata;
use crate::types::decode::Decode;
use crate::value::{PgValue, PgValueFormat, PgValueRef};
use rbdc::db::MetaData;
use rbdc::Error;
use rbs::Value;
use std::sync::Arc;

/// Implementation of [`Row`] for PostgreSQL.
#[derive(Debug)]
pub struct PgRow {
    pub(crate) data: DataRow,
    pub(crate) format: PgValueFormat,
    pub(crate) metadata: Arc<PgStatementMetadata>,
}

impl PgRow {
    pub fn columns(&self) -> &[PgColumn] {
        &self.metadata.columns
    }

    pub fn try_get_raw(&self, index: &str) -> Result<PgValueRef<'_>, Error> {
        let index = self.index(index)?;
        let column = &self.metadata.columns[index];
        let value = self.data.get(index);

        Ok(PgValueRef {
            format: self.format,
            type_info: column.type_info.clone(),
            value,
        })
    }

    pub fn try_take(&mut self, index: usize) -> Result<PgValue, Error> {
        if (index + 1) > self.metadata.column_names.len() {
            return Err(Error::from(format!("ColumnNotFound={}", index)));
        }
        let column = &self.metadata.columns[index];
        let value = self.data.take(index);
        Ok(PgValue {
            value: value,
            type_info: column.type_info.clone(),
            format: self.format,
        })
    }
}

impl PgRow {
    pub fn index(&self, idx: &str) -> Result<usize, Error> {
        self.metadata
            .column_names
            .get(idx)
            .ok_or_else(|| Error::from("ColumnNotFound=".to_string() + idx))
            .map(|v| *v)
    }
}

impl rbdc::db::Row for PgRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(PgMetaData {
            metadata: self.metadata.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Result<Value, Error> {
        match self.try_take(i) {
            Err(e) => Err(Error::from(format!("get error  index:{},error:{}", i, e))),
            Ok(v) => Value::decode(v),
        }
    }
}
