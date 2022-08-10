use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::types::Type;
use crate::{SqliteArgumentValue, SqliteTypeInfo, SqliteValue};
use rbdc::error::Error;

impl Type for String {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }
}

impl Encode for String {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Text(self));

        Ok(IsNull::No)
    }
}

impl Decode for String {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        value.text().map(ToOwned::to_owned)
    }
}
