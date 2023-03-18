use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use bigdecimal::BigDecimal;
use rbdc::decimal::Decimal;
use rbdc::Error;
use std::str::FromStr;

impl Encode for Decimal {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let b = BigDecimal::from_str(&self.0).map_err(|e| Error::from(e.to_string()))?;
        b.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for Decimal {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => Ok(Self::from(BigDecimal::decode(value)?.to_string())),
            PgValueFormat::Text => Ok(Self::from(value.as_str()?.to_string())),
        }
    }
}
