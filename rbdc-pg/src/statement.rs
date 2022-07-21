use crate::column::PgColumn;
use crate::type_info::PgTypeInfo;
use either::Either;
use rbdc::ext::ustr::UStr;
use rbdc::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PgStatement {
    pub(crate) sql: String,
    pub(crate) metadata: Arc<PgStatementMetadata>,
}

#[derive(Debug, Default)]
pub struct PgStatementMetadata {
    pub(crate) columns: Vec<PgColumn>,
    pub(crate) column_names: HashMap<UStr, usize>,
    pub(crate) parameters: Vec<PgTypeInfo>,
}

impl PgStatement {
    pub fn to_owned(&self) -> PgStatement {
        PgStatement {
            sql: self.sql.to_string(),
            metadata: self.metadata.clone(),
        }
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn parameters(&self) -> Option<Either<&[PgTypeInfo], usize>> {
        Some(Either::Left(&self.metadata.parameters))
    }

    pub fn columns(&self) -> &[PgColumn] {
        &self.metadata.columns
    }
}

impl PgStatement {
    pub fn index(&self, index: &str) -> Result<usize, Error> {
        self.metadata
            .column_names
            .get(index)
            .ok_or_else(|| Error::E(format!("ColumnNotFound {}", index)))
            .map(|v| *v)
    }
}
