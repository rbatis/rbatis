use std::str::FromStr;
use std::time::Duration;
use rbdc::datetime::FastDateTime;
use rbdc::Error;
use rbdc::timestamp::Timestamp;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};

impl Encode for FastDateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for FastDateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(fastdate::DateTime::decode(value)?))
    }
}

impl Decode for fastdate::DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime {
                    micro: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                    day: 1,
                    mon: 1,
                    year: 2000,
                };
                let us: i64 = Decode::decode(value)?;
                epoch + Duration::from_micros(us as u64)
            }
            PgValueFormat::Text => {
                //2022-07-22 05:22:22.123456+00
                fastdate::DateTime::from_str(value.as_str()?).map_err(|e| Error::from(e.to_string()))?
            }
        })
    }
}

impl Encode for fastdate::DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        Timestamp(self.unix_timestamp_millis() as u64).encode(buf)
    }
}