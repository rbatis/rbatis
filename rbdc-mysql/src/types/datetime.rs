use crate::types::date::decode_date_buf;
use crate::types::time::decode_time;
use crate::types::{Decode, Encode};
use crate::value::{MySqlValue, MySqlValueFormat};
use rbdc::datetime::DateTime;
use rbdc::Error;
use std::str::FromStr;
use fastdate::offset_sec;

impl Encode for DateTime {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let datetime = self.0;
        let datetime_size =
            date_time_size_hint(datetime.hour(), datetime.minu(), datetime.sec(), datetime.nano());
        buf.push(datetime_size as u8);
        let date = rbdc::date::Date(fastdate::Date {
            day: datetime.day(),
            mon: datetime.mon(),
            year: datetime.year(),
        });
        let mut size = date.encode(buf)?;
        buf.remove(buf.len() - (size + 1));
        if datetime_size > 4 {
            let time = fastdate::Time {
                nano: datetime.nano(),
                sec: datetime.sec(),
                minu: datetime.minu(),
                hour: datetime.hour(),
            };
            let size_time = time.encode(buf)?;
            buf.remove(buf.len() - (size_time + 1));
            size += size_time;
        }
        Ok(1 + size)
    }
}

impl Decode for DateTime {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(match value.format() {
            MySqlValueFormat::Text => Self(fastdate::DateTime::from_str(value.as_str()?)?),
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
                        minu: 0,
                        hour: 0,
                    }
                };
                let v = fastdate::DateTime::from((date, time)).set_offset(offset_sec());
                Self(v)
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
