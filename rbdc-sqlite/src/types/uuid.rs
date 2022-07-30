use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use rbdc::error::Error;
use crate::type_info::DataType;
use crate::{Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef};
use crate::types::Type;
use std::borrow::Cow;
use uuid::{fmt::Hyphenated, Uuid};

impl Type for Uuid {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Blob)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        matches!(ty.0, DataType::Blob | DataType::Text)
    }
}

impl Encode for Uuid {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>)->Result<IsNull,Error> {
        args.push(SqliteArgumentValue::Blob(Cow::Owned(
            self.as_bytes().to_vec(),
        )));

        Ok(IsNull::No)
    }
}

impl Decode<'_, Sqlite> for Uuid {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, Error> {
        // construct a Uuid from the returned bytes
        Uuid::from_slice(value.blob()).map_err(Into::into)
    }
}

impl Type for Hyphenated {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }
}

impl Encode for Hyphenated {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>)->Result<IsNull,Error> {
        args.push(SqliteArgumentValue::Text(Cow::Owned(self.to_string())));

        Ok(IsNull::No)
    }
}

impl Decode<'_, Sqlite> for Hyphenated {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, Error> {
        let uuid: Result<Uuid, Error> =
            Uuid::parse_str(&value.text().map(ToOwned::to_owned)?).map_err(Into::into);

        Ok(uuid?.hyphenated())
    }
}
