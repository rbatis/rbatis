use bytes::Buf;
use rbdc::types::time::Time;
use rbdc::Error;
use std::str::FromStr;

use crate::types::{Decode, Encode};
use crate::value::{MySqlValue, MySqlValueFormat};

impl Encode for Time {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        self.0.encode(buf)
    }
}

impl Decode for Time {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Time(fastdate::Time::decode(value)?))
    }
}

impl Encode for fastdate::Time {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let size = {
            if self.nano == 0 {
                3
            } else {
                7
            }
        };
        buf.push(size as u8);

        buf.push(self.hour as u8); //1
        buf.push(self.minu as u8); //1
        buf.push(self.sec as u8); //1
        if self.nano != 0 {
            buf.extend(self.get_micro().to_le_bytes()); //4
        }
        Ok(size)
    }
}

impl Decode for fastdate::Time {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(match value.format() {
            MySqlValueFormat::Text => fastdate::Time::from_str(value.as_str()?)?,
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;
                let len = buf[0];
                if len > 4 {
                    decode_time(len - 4, &buf[5..])
                } else {
                    fastdate::Time {
                        nano: 0,
                        sec: 0,
                        minu: 0,
                        hour: 0,
                    }
                }
            }
        })
    }
}

pub fn decode_time(len: u8, mut buf: &[u8]) -> fastdate::Time {
    let hour = buf.get_u8();
    let minute = buf.get_u8();
    let seconds = buf.get_u8();
    let micros = if len > 3 {
        // microseconds : int<EOF>
        buf.get_uint_le(buf.len())
    } else {
        0
    };
    // NaiveTime::from_hms_micro(hour as u32, minute as u32, seconds as u32, micros as u32)
    fastdate::Time {
        nano: micros as u32 * 1000,
        sec: seconds,
        minu: minute,
        hour,
    }
}
