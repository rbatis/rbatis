use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rbson::Bson;
use rbson::spec::BinarySubtype;
use chrono::Utc;
use serde::{Deserializer, Serializer};
use serde::de::Error;


#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateUtc {
    pub inner: chrono::NaiveDate,
}

impl From<chrono::NaiveDate> for DateUtc {
    fn from(arg: chrono::NaiveDate) -> Self {
        DateUtc {
            inner: arg
        }
    }
}

impl From<&chrono::NaiveDate> for DateUtc {
    fn from(arg: &chrono::NaiveDate) -> Self {
        DateUtc {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for DateUtc {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("DateUtc({})", self.inner));
        }else{
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateUtc {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("DateUtc(") && s.ends_with(")") {
                    let inner_data = &s["DateUtc(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::NaiveDate::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::NaiveDate::from_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}

impl std::fmt::Display for DateUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for DateUtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for DateUtc {
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DateUtc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl DateUtc {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> DateUtc {
        let utc = Utc::now();
        Self {
            inner: utc.date().naive_local(),
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::NaiveDate::from_str(arg)?;
        Ok(Self {
            inner: inner
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::DateUtc;

    #[test]
    fn test_ser_de() {
        let b = DateUtc::now();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: DateUtc = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}