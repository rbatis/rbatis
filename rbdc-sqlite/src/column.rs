use crate::{Sqlite, SqliteTypeInfo};
use rbdc::db::MetaData;
use rbdc::ext::ustr::UStr;
use rbs::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqliteColumn {
    pub(crate) name: UStr,
    pub(crate) ordinal: usize,
    pub(crate) type_info: SqliteTypeInfo,
}

impl SqliteColumn {
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn type_info(&self) -> &SqliteTypeInfo {
        &self.type_info
    }
}
