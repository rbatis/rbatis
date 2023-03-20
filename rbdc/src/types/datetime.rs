use crate::RBDCString;
use rbs::{to_value, Value};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;
use std::time::Duration;

#[deprecated(
    since = "4.1.0",
    note = "Please use `rbdc::datetime::DateTime` instead"
)]
pub type FastDateTime = DateTime;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct DateTime(pub fastdate::DateTime);

impl RBDCString for DateTime {
    fn ends_name() -> &'static str {
        "DT"
    }

    fn decode_str(arg: &str) -> Result<Self, crate::Error> {
        let is = Self::is(arg);
        if is != "" {
            return Ok(Self::from_str(arg.trim_end_matches(Self::ends_name()))?);
        }
        Err(crate::Error::E(format!(
            "warn type decode :{}",
            Self::ends_name()
        )))
    }
}

impl serde::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.0.to_string();
            s.push_str(Self::ends_name());
            serializer.serialize_str(&s)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            Value::String(mut v) => Ok({
                if std::any::type_name::<D>() == std::any::type_name::<rbs::Deserializer>() {
                    DateTime::trim_ends_match(&mut v);
                    DateTime::from(
                        fastdate::DateTime::from_str(&v)
                            .map_err(|e| D::Error::custom(e.to_string()))?,
                    )
                } else {
                    DateTime::from(
                        fastdate::DateTime::from_str(&v)
                            .map_err(|e| D::Error::custom(e.to_string()))?,
                    )
                }
            }),
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
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        self.0 = self.0.set_micro(micro);
        self
    }

    pub fn set_sec(mut self, sec: u8) -> Self {
        self.0 = self.0.set_sec(sec);
        self
    }

    pub fn set_min(mut self, min: u8) -> Self {
        self.0 = self.0.set_min(min);
        self
    }

    pub fn set_hour(mut self, hour: u8) -> Self {
        self.0 = self.0.set_hour(hour);
        self
    }

    pub fn set_day(mut self, day: u8) -> Self {
        self.0 = self.0.set_day(day);
        self
    }

    pub fn set_mon(mut self, mon: u8) -> Self {
        self.0 = self.0.set_mon(mon);
        self
    }

    pub fn set_year(mut self, year: u16) -> Self {
        self.0 = self.0.set_year(year);
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
        self.0 - rhs.0
    }
}

impl Add<Duration> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        DateTime::from(self.0.add(rhs))
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        DateTime::from(self.0.sub(rhs))
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
        Value::Map(rbs::value::map::ValueMap {
            inner: vec![
                ("type".into(), "DateTime".into()),
                ("value".into(), to_value!(arg.0)),
            ],
        })
    }
}

impl From<fastdate::DateTime> for DateTime {
    fn from(arg: fastdate::DateTime) -> Self {
        Self(arg)
    }
}

#[test]
fn test() {
    let date = DateTime::from_str("2017-02-06 00:00:00").unwrap();
    let v = rbs::to_value(&date).unwrap();
    println!("v={}", v);
    assert_eq!("\"2017-02-06 00:00:00DT\"", v.to_string());
    let date = DateTime::from(date.0);
    let v = rbs::to_value(&date).unwrap();
    println!("v={}", v);
    assert_eq!("\"2017-02-06 00:00:00DT\"", v.to_string());
}
