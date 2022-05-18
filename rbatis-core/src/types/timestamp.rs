use std::alloc::Layout;
use std::any::type_name;
use std::ops::{Deref, DerefMut};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use rbson::Bson;
use serde::{Deserializer, Serializer};
use serde::de::Error;
use crate::value::DateTimeNow;

/// Rbatis Timestamp
/// Rust type                Postgres type(s)
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
    pub inner: i64,
}

impl From<chrono::NaiveDateTime> for Timestamp {
    fn from(arg: chrono::NaiveDateTime) -> Self {
        Self {
            inner: arg.timestamp_millis()
        }
    }
}

impl From<&chrono::NaiveDateTime> for Timestamp {
    fn from(arg: &chrono::NaiveDateTime) -> Self {
        Self {
            inner: arg.timestamp_millis()
        }
    }
}

impl serde::Serialize for Timestamp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let bs = rbson::Timestamp {
            time: ((self.inner as u64) >> 32) as u32,
            increment: (self.inner & 0xFFFF_FFFF) as u32,
        };
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return bs.serialize(serializer);
        }else{
            return self.to_native_datetime().format("%Y-%m-%dT%H:%M:%S").to_string().serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                let s = NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%dT%H:%M:%S").or_else(|e| Err(D::Error::custom(e.to_string())))?;
                return Ok(Timestamp {
                    inner: s.timestamp_millis()
                });
            }
            Bson::Int64(data) => {
                return Ok(Timestamp::from_unix_timestamp(data));
            }
            Bson::UInt64(data) => {
                return Ok(Timestamp::from_unix_timestamp(data as i64));
            }
            Bson::Timestamp(data) => {
                return Ok(Timestamp::from(data));
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}


impl From<rbson::Timestamp> for Timestamp {
    fn from(data: rbson::Timestamp) -> Self {
        Self {
            inner: {
                let upper = (data.time.to_le() as u64) << 32;
                let lower = data.increment.to_le() as u64;
                (upper | lower) as i64
            }
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for Timestamp {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Timestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Timestamp> for chrono::NaiveDateTime {
    fn from(arg: Timestamp) -> Self {
        arg.to_native_datetime()
    }
}

impl Timestamp {
    pub fn now() -> Self {
        let offset_date_time = NaiveDateTime::now().timestamp_millis();
        Self {
            inner: offset_date_time
        }
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.inner
    }

    pub fn from_unix_timestamp(arg: i64) -> Self {
        Self {
            inner: arg
        }
    }

    pub fn to_native_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.inner / 1000, (self.inner % 1000 * 1000000) as u32)
    }
}


#[cfg(test)]
mod test {
    use crate::types::Timestamp;

    #[test]
    fn test_ser_de() {
        let b = Timestamp::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: Timestamp = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}
