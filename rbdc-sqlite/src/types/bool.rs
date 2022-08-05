use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValue};
use rbdc::error::Error;

impl Type for bool {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Bool)
    }
}

impl Encode for bool {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(i32::from(self)));

        Ok(IsNull::No)
    }
}

impl Decode for bool {
    fn decode(value: SqliteValue) -> Result<bool, Error> {
        Ok(value.int() != 0)
    }
}
