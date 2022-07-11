use crate::protocol::text::{ColumnFlags, ColumnType};
use rbdc::db::{MetaData, ResultSet};
use rbdc::ext::ustr::UStr;
use rbdc::Error;

pub struct MysqlResultSet {}

impl ResultSet for MysqlResultSet {
    fn meta_data(&self) -> Result<Box<dyn MetaData>, Error> {
        todo!()
    }

    fn next(&mut self) -> bool {
        todo!()
    }

    fn get(&self, i: u64) -> Result<rbs::value::Value, Error> {
        todo!()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MySqlColumn {
    pub ordinal: usize,
    pub name: UStr,
    pub type_info: MySqlTypeInfo,
    #[serde(skip)]
    pub flags: Option<ColumnFlags>,
}

/// Type information for a MySql type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MySqlTypeInfo {
    pub r#type: ColumnType,
    pub flags: ColumnFlags,
    pub char_set: u16,

    // [max_size] for integer types, this is (M) in BIT(M) or TINYINT(M)
    #[serde(default)]
    pub max_size: Option<u32>,
}
