use crate::protocol::text::{ColumnDefinition, ColumnFlags, ColumnType};
use rbdc::db::{MetaData, Row};
use rbdc::ext::ustr::UStr;
use rbdc::Error;

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
impl MySqlTypeInfo {
    pub(crate) const fn binary(ty: ColumnType) -> Self {
        Self {
            r#type: ty,
            flags: ColumnFlags::BINARY,
            char_set: 63,
            max_size: None,
        }
    }

    #[doc(hidden)]
    pub const fn __enum() -> Self {
        Self {
            r#type: ColumnType::Enum,
            flags: ColumnFlags::BINARY,
            char_set: 63,
            max_size: None,
        }
    }

    #[doc(hidden)]
    pub fn __type_feature_gate(&self) -> Option<&'static str> {
        match self.r#type {
            ColumnType::Date | ColumnType::Time | ColumnType::Timestamp | ColumnType::Datetime => {
                Some("time")
            }
            ColumnType::Json => Some("json"),
            ColumnType::NewDecimal => Some("bigdecimal"),
            _ => None,
        }
    }

    pub(crate) fn from_column(column: &ColumnDefinition) -> Self {
        Self {
            r#type: column.r#type,
            flags: column.flags,
            char_set: column.char_set,
            max_size: Some(column.max_size),
        }
    }
}
