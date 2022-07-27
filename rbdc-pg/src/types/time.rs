use std::fmt::{Display, Formatter};
use std::str::FromStr;
use bytes::Bytes;
use rbdc::Error;
use rbdc::common::time::Time;
use rbdc::timestamp::Timestamp;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};


impl Encode for Timestamp {
    fn encode(self, buf: &mut PgArgumentBuffer) -> IsNull {
        self.0.encode(buf)
    }
}

impl Decode for Timestamp {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                Timestamp(u64::decode(value)?)
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                Timestamp(fastdate::DateTime::from_str(s).unwrap().unix_timestamp_millis() as u64)
            }
        })
    }
}

impl Decode for Time{
     fn decode(value: PgValue) -> Result<Self, Error> {
         todo!()
     }
}