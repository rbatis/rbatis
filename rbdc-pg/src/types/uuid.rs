use rbdc::Error;
use rbdc::uuid::Uuid;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::PgValue;

impl Encode for Uuid{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        todo!()
    }
}

impl Decode for Uuid{
    fn decode(value: PgValue) -> Result<Self, Error> {
        todo!()
    }
}