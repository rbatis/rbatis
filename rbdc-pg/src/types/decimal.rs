use rbdc::decimal::Decimal;
use rbdc::Error;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};

impl Encode for Decimal{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for Decimal{
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => Ok(Self(String::from_utf8(value.into_bytes()?).unwrap_or("0".to_string()))),
            PgValueFormat::Text => Ok(Self(value.as_str()?.to_string())),
        }
    }
}