use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::value::IsNull;
use crate::value::{PgValueFormat, PgValueRef};
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::num::ParseIntError;

/// The PostgreSQL [`OID`] type stores an object identifier,
/// used internally by PostgreSQL as primary keys for various system tables.
///
/// [`OID`]: https://www.postgresql.org/docs/current/datatype-oid.html
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Oid(
    /// The raw unsigned integer value sent over the wire
    pub u32,
);

impl Oid {
    pub fn incr_one(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

impl Oid {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::OID
    }
}

impl Oid {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::OID_ARRAY
    }
}

impl Oid {
    pub fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&self.0.to_be_bytes());
        IsNull::No
    }
}

impl Oid {
    pub fn decode(value: PgValueRef<'_>) -> Result<Self, Error> {
        Ok(Self(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u32(value.as_bytes()?),
            PgValueFormat::Text => value
                .as_str()?
                .parse()
                .map_err(|e: ParseIntError| Error::from(e.to_string()))?,
        }))
    }
}

impl Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
