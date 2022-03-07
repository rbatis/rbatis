use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use chrono::Utc;
use serde::{Deserializer, Serializer};
use serde::de::Error;


#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeUtc {
    pub inner: chrono::NaiveTime,
}

impl From<chrono::NaiveTime> for TimeUtc {
    fn from(arg: chrono::NaiveTime) -> Self {
        Self {
            inner: arg
        }
    }
}

impl From<&chrono::NaiveTime> for TimeUtc {
    fn from(arg: &chrono::NaiveTime) -> Self {
        Self {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for TimeUtc {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("TimeUtc({})", self.inner));
        }else{
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for TimeUtc {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("TimeUtc(") && s.ends_with(")") {
                    let inner_data = &s["TimeUtc(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::NaiveTime::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::NaiveTime::from_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}

impl std::fmt::Display for TimeUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for TimeUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for TimeUtc {
    type Target = chrono::NaiveTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TimeUtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TimeUtc {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> TimeUtc {
        let utc = Utc::now();
        let dt = rbson::DateTime::from_millis(utc.timestamp_millis());
        Self {
            inner: dt.to_chrono().with_timezone(&Utc).time()
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::NaiveTime::from_str(arg)?;
        Ok(Self {
            inner: inner
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::TimeUtc;

    #[test]
    fn test_ser_de() {
        let b = TimeUtc::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: TimeUtc = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}