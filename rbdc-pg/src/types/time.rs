use std::fmt::{Display, Formatter};
use rbdc::Error;
use rbdc::common::time::Time;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};


impl Decode for Time{
     fn decode(value: PgValue) -> Result<Self, Error> {
         todo!()
     }
}

impl Encode for Time{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
       todo!()
    }
}