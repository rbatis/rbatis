use chrono::{Local, Utc};
use rbson::spec::BinarySubtype;
use rbson::Bson;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::any::type_name;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;
use std::time::SystemTime;

/// TimeLocal
/// Rust type              Postgres type(s)
/// chrono::NaiveTime      TIME
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeNative {
    pub inner: chrono::NaiveTime,
}

impl From<chrono::NaiveTime> for TimeNative {
    fn from(arg: chrono::NaiveTime) -> Self {
        Self { inner: arg }
    }
}

impl From<&chrono::NaiveTime> for TimeNative {
    fn from(arg: &chrono::NaiveTime) -> Self {
        Self { inner: arg.clone() }
    }
}

impl serde::Serialize for TimeNative {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("TimeNative({})", self.inner));
        } else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for TimeNative {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("TimeNative(") && s.ends_with(")") {
                    let inner_data = &s["TimeNative(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::NaiveTime::from_str(inner_data)
                            .or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::NaiveTime::from_str(&s)
                            .or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => Err(D::Error::custom("deserialize un supported bson type!")),
        }
    }
}

impl std::fmt::Display for TimeNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for TimeNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for TimeNative {
    type Target = chrono::NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TimeNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TimeNative {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> TimeNative {
        let utc = Local::now();
        let dt = rbson::DateTime::from_millis(utc.timestamp_millis());
        Self {
            inner: dt.to_chrono().with_timezone(&Local).time(),
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::NaiveTime::from_str(arg)?;
        Ok(Self { inner: inner })
    }
}

#[cfg(test)]
mod test {
    use crate::types::TimeNative;

    #[test]
    fn test_ser_de() {
        let b = TimeNative::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: TimeNative = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}
