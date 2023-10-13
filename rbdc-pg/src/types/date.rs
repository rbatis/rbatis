use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::date::Date;
use rbdc::Error;
use std::str::FromStr;
use std::time::Duration;

impl Decode for fastdate::Date {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::decode(value)?;
                let dt = fastdate::DateTime {
                    nano: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                    year: 2000,
                    day: 1,
                    mon: 1,
                    offset: fastdate::offset_sec(),
                };
                let dt = {
                    if days < 0 {
                        dt - Duration::from_secs((-days * 24 * 3600) as u64)
                    } else {
                        dt + Duration::from_secs((days * 24 * 3600) as u64)
                    }
                };
                fastdate::Date::from(dt)
            }

            PgValueFormat::Text => {
                let dt = fastdate::DateTime::from_str(&format!("{}T00:00:00", value.as_str()?))?;
                fastdate::Date::from(dt)
            }
        })
    }
}

impl Encode for fastdate::Date {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        // DATE is encoded as the days since epoch
        let days = (fastdate::DateTime {
            nano: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: self.day,
            mon: self.mon,
            year: self.year,
            offset: fastdate::offset_sec(),
        }
        .unix_timestamp_millis()
            - fastdate::DateTime {
                nano: 0,
                sec: 0,
                min: 0,
                hour: 0,
                year: 2000,
                day: 1,
                mon: 1,
            offset: fastdate::offset_sec(),
            }
            .unix_timestamp_millis())
            / (86400 * 1000) as i64;
        (days as i32).encode(buf)
    }
}

impl Decode for Date {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(fastdate::Date::decode(value)?))
    }
}

impl Encode for Date {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}
