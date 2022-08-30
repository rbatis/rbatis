use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::Error;
use rbs::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Timestamptz")]
pub struct Timestamptz(pub u64);

impl Display for Timestamptz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamptz({})", self.0)
    }
}

impl From<Timestamptz> for Value {
    fn from(arg: Timestamptz) -> Self {
        Value::Ext("Timestamptz", Box::new(Value::U64(arg.0)))
    }
}

impl Encode for Timestamptz {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}

impl Decode for Timestamptz {
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
                let v = epoch + std::time::Duration::from_micros(us as u64);
                Timestamptz(v.unix_timestamp_millis() as u64)
            }
            PgValueFormat::Text => {
                let s = value.as_str()?;
                Timestamptz(
                    fastdate::DateTime::from_str(s)
                        .unwrap()
                        .unix_timestamp_millis() as u64,
                )
            }
        })
    }
}
