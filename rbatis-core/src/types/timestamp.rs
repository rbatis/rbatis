use std::alloc::Layout;
use std::ops::{Deref, DerefMut};
use rbson::Bson;
use serde::{Deserializer, Serializer};
use serde::de::Error;
use sqlx_core::types::time;

/// Rbatis Timestamp
/// Rust type                Postgres type(s)
/// time::PrimitiveDateTime   TIMESTAMP
/// time::OffsetDateTime      TIMESTAMPTZ
///
/// Rust type                 MySQL type(s)
/// time::OffsetDateTime      TIMESTAMP
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
    pub inner: time::OffsetDateTime,
}

impl From<time::OffsetDateTime> for Timestamp {
    fn from(arg: time::OffsetDateTime) -> Self {
        Self {
            inner: arg
        }
    }
}

impl From<&time::OffsetDateTime> for Timestamp {
    fn from(arg: &time::OffsetDateTime) -> Self {
        Self {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for Timestamp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let bs = Timestamp::from_le_i64(self.inner.unix_timestamp());
        return bs.serialize(serializer);
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                return Ok(Self {
                    inner: time::OffsetDateTime::parse(&s, "%F %T %z").or_else(|e| Err(D::Error::custom(e.to_string())))?,
                });
            }
            Bson::Int64(data) => {
                return Ok(Timestamp::from_unix_timestamp(data));
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

impl Timestamp {
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

impl From<rbson::Timestamp> for Timestamp {
    fn from(data: rbson::Timestamp) -> Self {
        let offset = time::OffsetDateTime::from_unix_timestamp(Timestamp::as_timestamp(&data));
        Self {
            inner: offset
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
    type Target = time::OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Timestamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Timestamp {
    pub fn now() -> Self {
        let offset_date_time = time::OffsetDateTime::try_now_local().unwrap();

        Self {
            inner: time::OffsetDateTime::from_unix_timestamp(offset_date_time.unix_timestamp())
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = time::OffsetDateTime::parse(arg, "%F %T %z")?;
        Ok(Self {
            inner: inner
        })
    }

    pub fn timestamp_millis(&self) -> i64 {
        self.inner.unix_timestamp()
    }

    pub fn from_unix_timestamp(arg: i64) -> Self {
        Self {
            inner: time::OffsetDateTime::from_unix_timestamp(arg)
        }
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
