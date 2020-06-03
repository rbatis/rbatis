use std::str::from_utf8;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::{PgData, PgRawBuffer, PgTypeInfo, PgValue, Postgres};
use crate::types::Type;
use crate::Error;

impl Type<Postgres> for str {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::TEXT, "TEXT")
    }
}

impl Type<Postgres> for [&'_ str] {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::ARRAY_TEXT, "TEXT[]")
    }
}

impl Type<Postgres> for Vec<&'_ str> {
    fn type_info() -> PgTypeInfo {
        <[&'_ str] as Type<Postgres>>::type_info()
    }
}

impl Type<Postgres> for String {
    fn type_info() -> PgTypeInfo {
        <str as Type<Postgres>>::type_info()
    }
}

impl Type<Postgres> for [String] {
    fn type_info() -> PgTypeInfo {
        <[&'_ str] as Type<Postgres>>::type_info()
    }
}

impl Type<Postgres> for Vec<String> {
    fn type_info() -> PgTypeInfo {
        <Vec<&'_ str> as Type<Postgres>>::type_info()
    }
}

impl Encode<Postgres> for str {
    fn encode(&self, buf: &mut PgRawBuffer) {
        buf.extend_from_slice(self.as_bytes());
    }

    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl Encode<Postgres> for String {
    fn encode(&self, buf: &mut PgRawBuffer) {
        <str as Encode<Postgres>>::encode(self.as_str(), buf)
    }

    fn size_hint(&self) -> usize {
        self.len()
    }
}

impl<'de> Decode<'de, Postgres> for String {
    fn decode(value: PgValue<'de>) -> crate::Result<Self> {
        <&'de str as Decode<Postgres>>::decode(value).map(ToOwned::to_owned)
    }
}

impl<'de> Decode<'de, Postgres> for &'de str {
    fn decode(value: PgValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            PgData::Binary(buf) => from_utf8(buf).map_err(Error::decode),
            PgData::Text(s) => Ok(s),
        }
    }
}
