use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::type_info::DataType;
use crate::types::Type;
use crate::{Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValue, SqliteValueRef};
use rbdc::error::Error;

impl Type for i8 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl Encode for i8 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(self as i32));

        Ok(IsNull::No)
    }
}

impl Decode for i8 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int().try_into()?)
    }
}

impl Type for i16 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl Encode for i16 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(self as i32));

        Ok(IsNull::No)
    }
}

impl Decode for i16 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int().try_into()?)
    }
}

impl Type for i32 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl Encode for i32 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(self));

        Ok(IsNull::No)
    }
}

impl Decode for i32 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int())
    }
}

impl Type for i64 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int64)
    }
}

impl Encode for i64 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int64(self));

        Ok(IsNull::No)
    }
}

impl Decode for i64 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int64())
    }
}
