use rbs::Value;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FastDateTime(pub fastdate::DateTime);

impl Display for FastDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Serialize for FastDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_newtype_struct("DateTime", &self.0)
    }
}

impl<'de> Deserialize<'de> for FastDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        #[serde(rename = "DateTime")]
        pub struct DateTimeValue(pub Value);
        let v = DateTimeValue::deserialize(deserializer)?;
        match v.0 {
            Value::U64(u) => Ok(Self(fastdate::DateTime::from_timestamp_millis(u as i64))),
            Value::String(s) => Ok({
                Self(
                    fastdate::DateTime::from_str(&s)
                        .map_err(|e| D::Error::custom(e.to_string()))?,
                )
            }),
            _ => {
                return Err(D::Error::custom(&format!(
                    "unsupported type DateTime({})",
                    v.0
                )));
            }
        }
    }
}

impl Deref for FastDateTime {
    type Target = fastdate::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FastDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FastDateTime {
    pub fn now() -> Self {
        Self(fastdate::DateTime::now())
    }

    pub fn utc() -> Self {
        Self(fastdate::DateTime::utc())
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
        FastDateTime(fastdate::DateTime::from_timestamp(sec))
    }

    pub fn from_timestamp_millis(ms: i64) -> Self {
        FastDateTime(fastdate::DateTime::from_timestamp_millis(ms))
    }

    pub fn from_timestamp_nano(nano: u128) -> Self {
        FastDateTime(fastdate::DateTime::from_timestamp_nano(nano))
    }
}

impl Sub for FastDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Add<Duration> for FastDateTime {
    type Output = FastDateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        FastDateTime(self.0.add(rhs))
    }
}

impl Sub<Duration> for FastDateTime {
    type Output = FastDateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        FastDateTime(self.0.sub(rhs))
    }
}


impl FromStr for FastDateTime {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FastDateTime(
            fastdate::DateTime::from_str(s)
                .map_err(|e| crate::error::Error::from(e.to_string()))?,
        ))
    }
}

impl From<FastDateTime> for Value {
    fn from(arg: FastDateTime) -> Self {
        Value::Ext("DateTime", Box::new(Value::String(arg.0.to_string())))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "DateTime")]
pub struct DateTime(pub String);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Deref for DateTime {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for DateTime{
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime(s.to_string()))
    }
}

#[test]
fn test() {
    let date = DateTime("2017-02-06 00:00:00".to_string());
    let v = rbs::to_value(&date).unwrap();
    println!("{}", v);
    assert_eq!(
        "2017-02-06 00:00:00",
        v.as_str().unwrap_or_default().to_string()
    );
    let date = FastDateTime(fastdate::DateTime::from_str(&date.0).unwrap());
    let v = rbs::to_value(&date).unwrap();
    println!("{}", v);
    assert_eq!(
        "2017-02-06 00:00:00",
        v.as_str().unwrap_or_default().to_string()
    );
}
