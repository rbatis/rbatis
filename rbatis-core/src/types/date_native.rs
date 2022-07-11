use chrono::{Local, NaiveDate};
use rbson::spec::BinarySubtype;
use rbson::Bson;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Rust type              Postgres type(s)
/// chrono::NaiveDate      DATE
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateNative {
    pub inner: chrono::NaiveDate,
}

impl From<chrono::NaiveDate> for DateNative {
    fn from(arg: chrono::NaiveDate) -> Self {
        DateNative { inner: arg }
    }
}

impl From<&chrono::NaiveDate> for DateNative {
    fn from(arg: &chrono::NaiveDate) -> Self {
        DateNative { inner: arg.clone() }
    }
}

impl serde::Serialize for DateNative {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("DateNative({})", self.inner));
        } else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateNative {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("DateNative(") && s.ends_with(")") {
                    let inner_data = &s["DateNative(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::NaiveDate::from_str(inner_data)
                            .or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::NaiveDate::from_str(&s)
                            .or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => Err(D::Error::custom("deserialize un supported bson type!")),
        }
    }
}

impl std::fmt::Display for DateNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for DateNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for DateNative {
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DateNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl DateNative {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> DateNative {
        let utc = Local::now();
        Self {
            inner: utc.date().naive_local(),
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::NaiveDate::from_str(arg)?;
        Ok(Self { inner: inner })
    }
}

#[cfg(test)]
mod test {
    use crate::types::DateNative;

    #[test]
    fn test_ser_de() {
        let b = DateNative::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: DateNative = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}
