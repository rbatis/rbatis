use rbs::Value;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct DateTime(pub fastdate::DateTime);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_newtype_struct("DateTime", &self.0)
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        #[serde(rename = "DateTime")]
        pub struct DateTimeValue(pub Value);
        let v = DateTimeValue::deserialize(deserializer)?;
        match v.0 {
            Value::I32(u) => Ok(Self(fastdate::DateTime::from_timestamp_millis(u as i64))),
            Value::U32(u) => Ok(Self(fastdate::DateTime::from_timestamp_millis(u as i64))),
            Value::I64(u) => Ok(Self(fastdate::DateTime::from_timestamp_millis(u))),
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
        Self(fastdate::DateTime::now())
    }

    pub fn utc() -> Self {
        Self(fastdate::DateTime::utc())
    }

    pub fn from_timestamp(sec: i64) -> Self {
        DateTime(fastdate::DateTime::from_timestamp(sec))
    }

    pub fn from_timestamp_millis(ms: i64) -> Self {
        DateTime(fastdate::DateTime::from_timestamp_millis(ms))
    }

    pub fn from_timestamp_nano(nano: i128) -> Self {
        DateTime(fastdate::DateTime::from_timestamp_nano(nano))
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
        DateTime(self.0.add(rhs))
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        DateTime(self.0.sub(rhs))
    }
}

impl FromStr for DateTime {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime(
            fastdate::DateTime::from_str(s)
                .map_err(|e| crate::error::Error::from(e.to_string()))?,
        ))
    }
}

impl From<DateTime> for Value {
    fn from(arg: DateTime) -> Self {
        Value::Ext("DateTime", Box::new(Value::String(arg.0.to_string())))
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::datetime::DateTime;

    #[test]
    fn test_ser_de() {
        let dt = DateTime::now();
        let v = serde_json::to_value(&dt).unwrap();
        let new_dt: DateTime = serde_json::from_value(v).unwrap();
        assert_eq!(new_dt, dt);
    }

    #[test]
    fn test_de() {
        let dt = DateTime::from_str("2023-10-21T00:15:00.9233333+08:00").unwrap();
        println!("dt={}",dt);
        let v = serde_json::to_value(&dt).unwrap();
        let new_dt: DateTime = serde_json::from_value(v).unwrap();
        assert_eq!(new_dt, dt);
    }

    #[test]
    fn test_de2() {
        let dt = vec![DateTime::from_str("2023-10-21T00:15:00.9233333+08:00").unwrap()];
        let v = serde_json::to_value(&dt).unwrap();
        println!("dt={:?}",dt);
        let new_dt: Vec<DateTime> = serde_json::from_value(v).unwrap();
        assert_eq!(new_dt, dt);
    }

    #[test]
    fn test_de3() {
        let dt = vec![DateTime::from_str("2023-10-21T00:15:00.9233333+08:00").unwrap()];
        let v = rbs::to_value!(&dt);
        let new_dt: Vec<DateTime> = rbs::from_value(v).unwrap();
        assert_eq!(new_dt, dt);
    }
}