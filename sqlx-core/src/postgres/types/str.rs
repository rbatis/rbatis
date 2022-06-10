use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::postgres::types::array_compatible;
use crate::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef, Postgres};
use crate::types::Type;
use std::borrow::Cow;

impl Type<Postgres> for str {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::TEXT
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        [
            PgTypeInfo::TEXT,
            PgTypeInfo::NAME,
            PgTypeInfo::BPCHAR,
            PgTypeInfo::VARCHAR,
            PgTypeInfo::UNKNOWN,
        ]
        .contains(ty)
    }
}

impl Type<Postgres> for Cow<'_, str> {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl Type<Postgres> for String {
    fn type_info() -> PgTypeInfo {
        <&str as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <&str as Type<Postgres>>::compatible(ty)
    }
}

impl PgHasArrayType for &'_ str {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::TEXT_ARRAY
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        array_compatible::<&str>(ty)
    }
}

impl PgHasArrayType for Cow<'_, str> {
    fn array_type_info() -> PgTypeInfo {
        <&str as PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        <&str as PgHasArrayType>::array_compatible(ty)
    }
}

impl PgHasArrayType for String {
    fn array_type_info() -> PgTypeInfo {
        <&str as PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        <&str as PgHasArrayType>::array_compatible(ty)
    }
}

impl Encode<'_, Postgres> for &'_ str {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(self.as_bytes());

        IsNull::No
    }
}

impl Encode<'_, Postgres> for Cow<'_, str> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Cow::Borrowed(str) => <&str as Encode<Postgres>>::encode(*str, buf),
            Cow::Owned(str) => <&str as Encode<Postgres>>::encode(&**str, buf),
        }
    }
}

impl Encode<'_, Postgres> for String {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        <&str as Encode<Postgres>>::encode(&**self, buf)
    }
}

impl<'r> Decode<'r, Postgres> for &'r str {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?)
    }
}

impl<'r> Decode<'r, Postgres> for Cow<'r, str> {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Cow::Borrowed(value.as_str()?))
    }
}

impl Decode<'_, Postgres> for String {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        Ok(value.as_str()?.to_owned())
    }
}
