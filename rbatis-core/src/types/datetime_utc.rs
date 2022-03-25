use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use serde::{Deserializer, Serializer};
use chrono::Utc;
use serde::de::Error;

/// Rbatis DateTime Utc
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTimeUtc {
    pub inner: chrono::DateTime<Utc>,
}

impl From<chrono::DateTime<Utc>> for DateTimeUtc {
    fn from(arg: chrono::DateTime<Utc>) -> Self {
        DateTimeUtc {
            inner: arg
        }
    }
}

impl From<&chrono::DateTime<Utc>> for DateTimeUtc {
    fn from(arg: &chrono::DateTime<Utc>) -> Self {
        DateTimeUtc {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for DateTimeUtc {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("DateTimeUtc({})", self.inner));
        }else{
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateTimeUtc {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::DateTime(date) => {
                return Ok(Self {
                    inner: date.to_chrono(),
                });
            }
            Bson::String(s) => {
                let mut b = s.into_bytes();
                if b.len() >= 10 && b[10] == ' ' as u8 {
                    b[10] = 'T' as u8;
                }
                let s = unsafe { String::from_utf8_unchecked(b) };
                if s.starts_with("DateTimeUtc(") && s.ends_with(")") {
                    let inner_data = &s["DateTimeUtc(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::DateTime::<chrono::Utc>::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::DateTime::<chrono::Utc>::from_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}

impl std::fmt::Display for DateTimeUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for DateTimeUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for DateTimeUtc {
    type Target = chrono::DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DateTimeUtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl DateTimeUtc {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> DateTimeUtc {
        let utc = Utc::now();
        let dt = rbson::DateTime::from_millis(utc.timestamp_millis());
        Self {
            inner: dt.to_chrono()
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::DateTime::<Utc>::from_str(arg)?;
        Ok(Self {
            inner: inner
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::DateTimeUtc;

    #[test]
    fn test_ser_de() {
        let b = DateTimeUtc::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: DateTimeUtc = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}