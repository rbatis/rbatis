use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::datetime::DateTime;
use rbdc::timestamp::Timestamp;
use rbdc::Error;
use std::str::FromStr;
use std::time::Duration;

impl Encode for DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Encode for fastdate::DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        Timestamp(self.unix_timestamp_millis()).encode(buf)
    }
}
