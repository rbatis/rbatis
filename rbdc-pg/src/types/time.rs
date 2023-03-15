use rbdc::common::time::Time;
use rbdc::Error;
use std::str::FromStr;
use std::time::Duration;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};

impl Decode for Time {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us = i64::decode(value)?;
                //+microseconds
                let t = fastdate::DateTime {
                    nano: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                    day: 0,
                    mon: 0,
                    year: 0,
                };
                let t = {
                    if us < 0 {
                        t - Duration::from_micros(-us as u64)
                    } else {
                        t + Duration::from_micros(us as u64)
                    }
                };
                Ok(Time::from(fastdate::Time {
                    nano: t.nano,
                    sec: t.sec,
                    min: t.min,
                    hour: t.hour,
                }))
            }
            PgValueFormat::Text => Ok(Time::from(fastdate::Time::from_str(value.as_str()?)?)),
        }
    }
}

impl Encode for Time {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        // TIME is encoded as the microseconds since midnight
        // microseconds
        let us = self.value.get_micro()
            + self.value.hour as u32 * 60 * 60 * 1000000
            + self.value.min as u32 * 60 * 1000000
            + self.value.sec as u32 * 1000000;
        us.encode(buf)
    }
}
