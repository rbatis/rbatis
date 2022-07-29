use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::TypeInfo;
use crate::value::{PgValue, PgValueFormat};

impl Decode for u64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u16 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u16(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u8 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(value.as_bytes()?.get(0).copied().unwrap_or_default() as u8)
    }
}

impl Decode for i64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i32(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i16 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i16(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i8 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(value.as_bytes()?.get(0).copied().unwrap_or_default() as i8)
    }
}

///encode

impl Encode for u64 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());
        Ok(IsNull::No)
    }
}

impl Encode for u32 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for u16 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for u8 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i64 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i32 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i16 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i8 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

///TypeInfo

impl TypeInfo for i8{
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::BYTEA
    }
}
