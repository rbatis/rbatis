use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ReadBytesExt};
use fastdate::{DateTime, offset_sec};
use rbdc::Error;
use rbs::Value;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::str::FromStr;
use serde::Deserializer;

/// (timestamp,offset sec)
#[derive(serde::Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Timestamptz")]
pub struct Timestamptz(pub i64, pub i32);

impl<'de> serde::Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
        #[serde(rename = "Timestamptz")]
        struct Timestamptz1(pub i64, pub i32);

        let v = Value::deserialize(deserializer)?;
        match v {
            Value::String(v) => {
                let date = fastdate::DateTime::from_str(&v).map_err(|e| D::Error::custom(e.to_string()))?;
                Ok(Timestamptz(date.unix_timestamp_millis(), date.offset()))
            }
            Value::Array(v) => {
                if v.len() == 2 && v[0].is_i64() && v[1].is_i32() {
                    let r: Timestamptz1 = rbs::from_value(Value::Ext("Timestamptz", Box::new(Value::Array(v)))).map_err(|e| D::Error::custom(e.to_string()))?;
                    Ok(Self(r.0, r.1))
                } else {
                    Err(D::Error::custom("Timestamptz must be String or Value::Ext(\"Timestamptz\", Array([I64(0), I32(0)]))"))
                }
            }
            Value::Ext(t, v) => {
                let v: Timestamptz1 = rbs::from_value(Value::Ext(t, v)).map_err(|e| D::Error::custom(e.to_string()))?;
                Ok(Self(v.0, v.1))
            }
            _ => {
                Err(D::Error::custom("Timestamptz must be String or Value::Ext(\"Timestamptz\", Array([I64(0), I32(0)]))"))
            }
        }
    }
}

impl Timestamptz {
    pub fn now() -> Self {
        let now = fastdate::DateTime::now();
        Self::from(now)
    }

    pub fn utc() -> Self {
        let now = fastdate::DateTime::utc();
        Self::from(now)
    }
}

impl From<fastdate::DateTime> for Timestamptz {
    fn from(date: DateTime) -> Self {
        Timestamptz(date.unix_timestamp_millis(), date.offset())
    }
}

impl Display for Timestamptz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fastdate::DateTime::from_timestamp_millis(self.0 as i64).set_offset(self.1)
        )
    }
}

impl From<Timestamptz> for Value {
    fn from(arg: Timestamptz) -> Self {
        Value::Ext(
            "Timestamptz",
            Box::new(Value::Array(vec![Value::I64(arg.0), Value::I32(arg.1)])),
        )
    }
}

impl Encode for Timestamptz {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let epoch = fastdate::DateTime::from(fastdate::Date {
            day: 1,
            mon: 1,
            year: 2000,
        });
        let dt = fastdate::DateTime::from_timestamp_millis(self.0);
        let micros;
        if dt >= epoch {
            micros = (dt - epoch).as_micros() as i64;
        } else {
            micros = (epoch - dt).as_micros() as i64 * -1;
        }
        micros.encode(buf)
    }
}

impl Decode for Timestamptz {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let mut buf = Cursor::new(value.as_bytes()?);
                // TIME is encoded as the microseconds since midnight
                let us = buf.read_i64::<BigEndian>()?;
                // TODO offset_seconds from connection params
                let offset_seconds = offset_sec();
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
                Timestamptz(v.unix_timestamp_millis(), offset_seconds)
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                let date = fastdate::DateTime::from_str(s)?;
                Timestamptz::from(date)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::timestamptz::Timestamptz;

    #[test]
    fn test_de() {
        let tz = Timestamptz(1, 0);
        let v = rbs::to_value(&tz).unwrap();
        println!("{:?}", v);
        let r: Timestamptz = rbs::from_value(v).unwrap();
        assert_eq!(r, tz);
    }

    //2024-07-26 09:03:48+00
    #[test]
    fn test_de_date() {
        let v = rbs::Value::String("2024-07-26 09:03:48+00".to_string());
        println!("{:?}", v);
        let r: Timestamptz = rbs::from_value(v).unwrap();
    }
}
