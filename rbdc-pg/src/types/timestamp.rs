use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::timestamp::Timestamp;
use rbdc::Error;
use std::str::FromStr;

impl Encode for Timestamp {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let epoch = fastdate::DateTime::from(fastdate::Date {
            day: 1,
            mon: 1,
            year: 2000,
        });
        let dt = fastdate::DateTime::from_timestamp_millis(self.0);
        let  micros;
        if dt >= epoch{
            micros = (dt - epoch).as_micros() as i64;
        }else{
            micros = (epoch - dt).as_micros() as i64 * -1;
        }
        micros.encode(buf)
    }
}

impl Decode for Timestamp {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime::from(fastdate::Date {
                    day: 1,
                    mon: 1,
                    year: 2000,
                });
                let us: i64 = Decode::decode(value)?;
                let v = {
                    if us < 0 {
                        epoch - std::time::Duration::from_micros(-us as u64)
                    } else {
                        epoch + std::time::Duration::from_micros(us as u64)
                    }
                };
                Timestamp(v.unix_timestamp_millis())
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                Timestamp(fastdate::DateTime::from_str(s)?.unix_timestamp_millis())
            }
        })
    }
}
