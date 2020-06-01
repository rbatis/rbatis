use crate::decode::Decode;
use crate::encode::Encode;
use crate::mysql::protocol::TypeId;
use crate::mysql::type_info::MySqlTypeInfo;
use crate::mysql::{MySql, MySqlData, MySqlValue};
use crate::types::Type;
use serde::Serialize;

impl Type<MySql> for serde_json::Value {
    fn type_info() -> MySqlTypeInfo {
        MySqlTypeInfo::new(TypeId::TINY_INT)
    }
}

