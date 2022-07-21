use crate::column::PgColumn;
use crate::message::DataRow;
use crate::statement::PgStatementMetadata;
use crate::value::{PgValueFormat, PgValueRef};
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
            row: Some(&self.data.storage),
            type_info: column.type_info.clone(),
            value,
        })
    }
}

impl PgRow {
    pub fn index(&self, idx: &str) -> Result<usize, Error> {
        self.metadata
            .column_names
            .get(idx)
            .ok_or_else(|| Error::E("ColumnNotFound=".to_string() + idx))
            .map(|v| *v)
    }
}

impl rbdc::db::Row for PgRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        todo!()
    }

    fn get(&mut self, i: usize) -> Option<Value> {
        todo!()
    }
}
