use std::io::Cursor;
use std::str::FromStr;
use byteorder::{BigEndian, ReadBytesExt};
use crate::arguments::PgArgumentBuffer;
use crate::types::encode::{Encode, IsNull};
use rbdc::datetime::DateTime;
use rbdc::Error;
use crate::types::decode::Decode;
use crate::value::{PgValue, PgValueFormat};

/// Encode to Timestamptz
impl Encode for DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let millis= self.unix_timestamp_millis();
        let epoch = fastdate::DateTime::from(fastdate::Date {
            day: 1,
            mon: 1,
            year: 2000,
        });
        let dt = fastdate::DateTime::from_timestamp_millis(millis);
        let micros;
        if dt >= epoch {
            micros = (dt - epoch).as_micros() as i64;
        } else {
            micros = (epoch - dt).as_micros() as i64 * -1;
        }
        micros.encode(buf)
    }
}

impl Decode for DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let mut buf = Cursor::new(value.as_bytes()?);
                // TIME is encoded as the microseconds since midnight
                let us = buf.read_i64::<BigEndian>()?;
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime::from(fastdate::Date {
                    day: 1,
                    mon: 1,
                    year: 2000,
                });
                let v = {
                    if us < 0 {
                        epoch - std::time::Duration::from_micros(-us as u64)
                    } else {
                        epoch + std::time::Duration::from_micros(us as u64)
                    }
                };
                DateTime(fastdate::DateTime::from_timestamp_millis(v.unix_timestamp_millis()))
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                let date = fastdate::DateTime::from_str(s)?;
                DateTime(date)
            }
        })
    }
}
