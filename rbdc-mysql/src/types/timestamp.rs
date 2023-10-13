use std::str::FromStr;

use crate::types::date::decode_date_buf;
use crate::types::time::decode_time;
use crate::types::{Decode, Encode};
use crate::value::{MySqlValue, MySqlValueFormat};
use rbdc::timestamp::Timestamp;
use rbdc::Error;

impl Encode for Timestamp {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let datetime = fastdate::DateTime::from_timestamp_millis(self.0 as i64);
        let size = date_time_size_hint(datetime.hour, datetime.min, datetime.sec, datetime.nano);
        buf.push(size as u8);
        let date = fastdate::Date {
            day: datetime.day,
            mon: datetime.mon,
            year: datetime.year,
        };
        let size_date = date.encode(buf)?;
        buf.remove(buf.len() - 1 - size_date);
        let mut size_time = 0;
        if (size + size_date) > 4 {
            let time = fastdate::Time {
                nano: datetime.nano,
                sec: datetime.sec,
                min: datetime.min,
                hour: datetime.hour,
            };
            size_time = time.encode(buf)?;
            buf.remove(buf.len() - 1 - size_time);
        }
        Ok(size + size_date + size_time)
    }
}

impl Decode for Timestamp {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(match value.format() {
            MySqlValueFormat::Text => {
                Self(fastdate::DateTime::from_str(value.as_str()?)?.unix_timestamp_millis() as u64)
            }
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;
                let len = buf[0];
                let date = decode_date_buf(&buf[1..])?;
                let time = if len > 4 {
                    decode_time(len - 4, &buf[5..])
                } else {
                    fastdate::Time {
                        nano: 0,
                        sec: 0,
                        min: 0,
                        hour: 0,
                    }
                };
                Self(
                    fastdate::DateTime {
                        nano: time.nano,
                        sec: time.sec,
                        min: time.min,
                        hour: time.hour,
                        day: date.day,
                        mon: date.mon,
                        year: date.year,
                        offset: fastdate::offset_sec(),
                    }
                    .unix_timestamp_millis() as u64,
                )
            }
        })
    }
}

fn date_time_size_hint(hour: u8, min: u8, sec: u8, nano: u32) -> usize {
    // to save space the packet can be compressed:
    match (hour, min, sec, nano) {
        // if hour, minutes, seconds and micro_seconds are all 0,
        // length is 4 and no other field is sent
        (0, 0, 0, 0) => 4,

        // if micro_seconds is 0, length is 7
        // and micro_seconds is not sent
        (_, _, _, 0) => 7,

        // otherwise length is 11
        (_, _, _, _) => 11,
    }
}
