use serde::{Deserialize, Serialize};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use rbdc::error::Error;
use crate::{
    type_info::DataType, Sqlite, SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef,
};
use crate::types::{Json, Type};

impl<T> Type for Json<T> {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }

    fn compatible(ty: &SqliteTypeInfo) -> bool {
        <&str as Type>::compatible(ty)
    }
}

impl<T> Encode<'_, Sqlite> for Json<T>
where
    T: Serialize,
{
    fn encode(&self, buf: &mut Vec<SqliteArgumentValue>) -> IsNull {
        let json_string_value =
            serde_json::to_string(&self.0).expect("serde_json failed to convert to string");

        Encode::<Sqlite>::encode(json_string_value, buf)
    }
}

impl<'r, T> Decode for Json<T>
where
    T: 'r + Deserialize<'r>,
{
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, Error> {
        let string_value = <&str as Decode<Sqlite>>::decode(value)?;

        serde_json::from_str(&string_value)
            .map(Json)
            .map_err(Into::into)
    }
}
