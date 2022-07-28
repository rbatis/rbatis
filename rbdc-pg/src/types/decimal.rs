use rbdc::decimal::Decimal;
use rbdc::Error;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::PgValue;

impl Encode for Decimal{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        todo!()
    }
}

impl Decode for Decimal{
    fn decode(value: PgValue) -> Result<Self, Error> {
        todo!()
    }
}