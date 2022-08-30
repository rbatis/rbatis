use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::timestamp::Timestamp;
use rbdc::Error;
use std::str::FromStr;

impl Encode for Timestamp {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let v = 1000*(self.0 as i64 - fastdate::DateTime::from(fastdate::Date{
            day: 1,
            mon: 1,
            year: 2000
        }).unix_timestamp_millis());
        v.encode(buf)
    }
}

impl Decode for Timestamp {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime::from(fastdate::Date{
                    day: 1,
                    mon: 1,
                    year: 2000
                });
                let us: i64 = Decode::decode(value)?;
                let v = epoch + std::time::Duration::from_micros(us as u64);
                Timestamp(v.unix_timestamp_millis() as u64)
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                Timestamp(
                    fastdate::DateTime::from_str(s)
                        .unwrap()
                        .unix_timestamp_millis() as u64,
                )
            }
        })
    }
}
