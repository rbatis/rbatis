use rbs::{to_value, Value};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Index, Sub};
use std::str::FromStr;
use std::time::Duration;

#[deprecated(
    since = "4.1.0",
    note = "Please use `rbdc::datetime::DateTime` instead"
)]
pub type FastDateTime = DateTime;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "DateTime")]
pub struct DateTime {
    pub r#type: &'static str,
    pub value: fastdate::DateTime,
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        match v {
            Value::I32(u) => Ok(DateTime::from(fastdate::DateTime::from_timestamp_millis(
                u as i64,
            ))),
            Value::U32(u) => Ok(DateTime::from(fastdate::DateTime::from_timestamp_millis(
                u as i64,
            ))),
            Value::I64(u) => Ok(DateTime::from(fastdate::DateTime::from_timestamp_millis(u))),
            Value::U64(u) => Ok(DateTime::from(fastdate::DateTime::from_timestamp_millis(
                u as i64,
            ))),
            Value::String(s) => Ok({
                DateTime::from(
                    fastdate::DateTime::from_str(&s)
                        .map_err(|e| D::Error::custom(e.to_string()))?,
                )
            }),
            Value::Map(mut v) => {
                let t = v.index("type").as_str().unwrap_or_default();
                if t == "DateTime" {
                    Ok(DateTime::from(
                        fastdate::DateTime::from_str(v.rm("value").as_str().unwrap_or_default())
                            .map_err(|e| D::Error::custom(e.to_string()))?,
                    ))
                } else {
                    Err(D::Error::custom(&format!(
                        "unsupported type DateTime({})",
                        v
                    )))
                }
            }
            _ => {
                return Err(D::Error::custom(&format!(
                    "unsupported type DateTime({})",
                    v
                )));
            }
        }
    }
}

impl Deref for DateTime {
    type Target = fastdate::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self::from(fastdate::DateTime::now())
    }

    pub fn utc() -> Self {
        Self::from(fastdate::DateTime::utc())
    }

    pub fn set_micro(mut self, micro: u32) -> Self {
        self.value = self.value.set_micro(micro);
        self
    }

    pub fn set_sec(mut self, sec: u8) -> Self {
        self.value = self.value.set_sec(sec);
        self
    }

    pub fn set_min(mut self, min: u8) -> Self {
        self.value = self.value.set_min(min);
        self
    }

    pub fn set_hour(mut self, hour: u8) -> Self {
        self.value = self.value.set_hour(hour);
        self
    }

    pub fn set_day(mut self, day: u8) -> Self {
        self.value = self.value.set_day(day);
        self
    }

    pub fn set_mon(mut self, mon: u8) -> Self {
        self.value = self.value.set_mon(mon);
        self
    }

    pub fn set_year(mut self, year: u16) -> Self {
        self.value = self.value.set_year(year);
        self
    }

    pub fn from_timestamp(sec: i64) -> Self {
        Self::from(fastdate::DateTime::from_timestamp(sec))
    }

    pub fn from_timestamp_millis(ms: i64) -> Self {
        Self::from(fastdate::DateTime::from_timestamp_millis(ms))
    }

    pub fn from_timestamp_nano(nano: u128) -> Self {
        Self::from(fastdate::DateTime::from_timestamp_nano(nano))
    }
}

impl Sub for DateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.value - rhs.value
    }
}

impl Add<Duration> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        DateTime::from(self.value.add(rhs))
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        DateTime::from(self.value.sub(rhs))
    }
}

impl FromStr for DateTime {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime::from(
            fastdate::DateTime::from_str(s)
                .map_err(|e| crate::error::Error::from(e.to_string()))?,
        ))
    }
}

impl From<DateTime> for Value {
    fn from(arg: DateTime) -> Self {
        Value::Map(rbs::value::map::ValueMap{
            inner: vec![("type".into(),"DateTime".into()),("value".into(),to_value!(arg.value))],
        })
    }
}

impl From<fastdate::DateTime> for DateTime {
    fn from(arg: fastdate::DateTime) -> Self {
        Self {
            r#type: "DateTime",
            value: arg,
        }
    }
}

#[test]
fn test() {
    let date = DateTime::from_str("2017-02-06 00:00:00").unwrap();
    let v = rbs::to_value(&date).unwrap();
    println!("{}", v);
    assert_eq!(
        r#"{"type":"DateTime","value":"2017-02-06 00:00:00"}"#,
        v.to_string()
    );
    let date = DateTime::from(date.value);
    let v = rbs::to_value(&date).unwrap();
    println!("{}", v);
    assert_eq!(
        r#"{"type":"DateTime","value":"2017-02-06 00:00:00"}"#,
        v.to_string()
    );
}
