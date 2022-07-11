use crate::value::DateTimeNow;
use chrono::{NaiveDateTime, Utc};
use rbson::spec::BinarySubtype;
use rbson::Bson;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::alloc::Layout;
use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Rbatis Timestamp
/// Rust type                Postgres type(s)
/// time::OffsetDateTime      TIMESTAMPTZ
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimestampZ {
    pub inner: chrono::DateTime<Utc>,
}

impl From<chrono::DateTime<Utc>> for TimestampZ {
    fn from(arg: chrono::DateTime<Utc>) -> Self {
        Self { inner: arg }
    }
}

impl From<&chrono::DateTime<Utc>> for TimestampZ {
    fn from(arg: &chrono::DateTime<Utc>) -> Self {
        Self { inner: arg.clone() }
    }
}

impl serde::Serialize for TimestampZ {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("TimestampZ({})", self.inner.to_string()));
        } else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for TimestampZ {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("TimestampZ(") && s.ends_with(")") {
                    let inner_data = &s["TimestampZ(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: {
                            match chrono::DateTime::from_str(inner_data) {
                                Ok(v) => Ok(v),
                                Err(e) => {
                                    log::error!("{}", e);
                                    Err(D::Error::custom("parse TimestampZ fail"))
                                }
                            }
                        }?,
                    });
                } else {
                    return Ok(Self {
                        inner: {
                            match chrono::DateTime::from_str(&s) {
                                Ok(v) => Ok(v),
                                Err(e) => {
                                    log::error!("{}", e);
                                    Err(D::Error::custom("parse TimestampZ fail"))
                                }
                            }
                        }?,
                    });
                }
            }
            _ => Err(D::Error::custom("deserialize un supported bson type!")),
        }
    }
}

impl TimestampZ {
    pub fn as_timestamp(arg: &rbson::Timestamp) -> i64 {
        let upper = (arg.time.to_le() as u64) << 32;
        let lower = arg.increment.to_le() as u64;
        (upper | lower) as i64
    }

    pub fn from_le_i64(val: i64) -> rbson::Timestamp {
        let ts = val.to_le();
        rbson::Timestamp {
            time: ((ts as u64) >> 32) as u32,
            increment: (ts & 0xFFFF_FFFF) as u32,
        }
    }
}

impl std::fmt::Display for TimestampZ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for TimestampZ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for TimestampZ {
    type Target = chrono::DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TimestampZ {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TimestampZ {
    pub fn now() -> Self {
        Self {
            inner: chrono::DateTime::from_utc(NaiveDateTime::now(), Utc),
        }
    }

    pub fn now_utc() -> Self {
        Self {
            inner: chrono::DateTime::from_utc(NaiveDateTime::now(), Utc),
        }
    }

    pub fn now_local() -> Self {
        Self {
            inner: chrono::DateTime::from_utc(NaiveDateTime::now(), Utc),
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        Ok(Self {
            inner: chrono::DateTime::<Utc>::from_str(arg)?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::TimestampZ;

    #[test]
    fn test_native() {
        let dt = TimestampZ::now_utc();
        let s = rbson::to_bson(&dt).unwrap();
        let dt_new: TimestampZ = rbson::from_bson(s).unwrap();
        println!("{},{}", dt.timestamp_millis(), dt_new.timestamp_millis());
        assert_eq!(dt, dt_new);
    }

    #[test]
    fn test_ser_de() {
        let b = TimestampZ::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: TimestampZ = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }

    #[test]
    fn test_str_de() {
        let b = "2022-01-05 03:18:49 UTC".to_string();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: TimestampZ = rbson::from_bson(bsons).unwrap();
        //assert_eq!(b, b_de);
    }
}
