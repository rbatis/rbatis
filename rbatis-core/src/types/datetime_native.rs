use std::any::{Any, type_name};
use std::fmt::Debug;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;
use std::time::SystemTime;
use rbson::{Bson, DateTime};
use rbson::spec::BinarySubtype;
use chrono::{Local, NaiveDateTime, Utc};
use serde::{Deserializer, Serializer};
use serde::de::Error;
use crate::value::DateTimeNow;

/// Rbatis DateTime
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTimeNative {
    pub inner: chrono::NaiveDateTime,
}

impl From<rbson::DateTime> for DateTimeNative {
    fn from(arg: DateTime) -> Self {
        Self {
            inner: arg.to_chrono().with_timezone(&chrono::Local).naive_local(),
        }
    }
}

impl From<chrono::DateTime<Local>> for DateTimeNative {
    fn from(arg: chrono::DateTime<Local>) -> Self {
        DateTimeNative {
            inner: arg.naive_local()
        }
    }
}

impl From<&chrono::DateTime<Local>> for DateTimeNative {
    fn from(arg: &chrono::DateTime<Local>) -> Self {
        DateTimeNative {
            inner: arg.clone().naive_local()
        }
    }
}

impl serde::Serialize for DateTimeNative {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        struct FormatWrapped<'a, D: 'a> {
            inner: &'a D,
        }
        impl<'a, D: std::fmt::Debug> std::fmt::Display for FormatWrapped<'a, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("DateTimeNative({})", FormatWrapped { inner: &self }.to_string()));
        } else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for DateTimeNative {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::DateTime(date) => {
                return Ok(Self {
                    inner: date.to_chrono().with_timezone(&chrono::Local).naive_local(),
                });
            }
            Bson::String(s) => {
                let mut b = s.into_bytes();
                if b.len() >= 10 && b[10] == ' ' as u8 {
                    b[10] = 'T' as u8;
                }
                let s = unsafe { String::from_utf8_unchecked(b) };
                if s.starts_with("DateTimeNative(") && s.ends_with(")") {
                    let inner_data = &s["DateTimeNative(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: chrono::NaiveDateTime::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: chrono::NaiveDateTime::from_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            //timestamp(ms)
            Bson::Int64(timestamp) => {
                return Ok(DateTimeNative {
                    inner: chrono::NaiveDateTime::from_timestamp(timestamp/1000, (timestamp % 1000 * 1000000) as u32)
                });
            }
            v => {
                Err(D::Error::custom(format!("deserialize un supported bson value={:?},type= DateTimeNative", v)))
            }
        }
    }
}

impl std::fmt::Display for DateTimeNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct FormatWrapped<'a, D: 'a> {
            inner: &'a D,
        }
        impl<'a, D: std::fmt::Debug> std::fmt::Display for FormatWrapped<'a, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }
        FormatWrapped { inner: &self.inner }.fmt(f)
    }
}

impl std::fmt::Debug for DateTimeNative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct FormatWrapped<'a, D: 'a> {
            inner: &'a D,
        }
        impl<'a, D: std::fmt::Debug> std::fmt::Debug for FormatWrapped<'a, D> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }
        FormatWrapped { inner: &self.inner }.fmt(f)
    }
}

impl Deref for DateTimeNative {
    type Target = chrono::NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for DateTimeNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl DateTimeNative {
    /// Returns a [`DateTime`] which corresponds to the current date and time.
    pub fn now() -> DateTimeNative {
        Self {
            inner: chrono::NaiveDateTime::now()
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = chrono::NaiveDateTime::from_str(arg)?;
        Ok(Self {
            inner: inner
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::{DateTimeNative};

    #[test]
    fn test_native() {
        let dt = DateTimeNative::now();
        let s = rbson::to_bson(&dt).unwrap();
        let dt_new: DateTimeNative = rbson::from_bson(s).unwrap();
        println!("{},{}", dt.timestamp_millis(), dt_new.timestamp_millis());
        assert_eq!(dt, dt_new);
    }

    #[test]
    fn test_utc() {
        let dt = DateTimeNative::now();
        println!("{}", dt);
        let s = rbson::to_bson(&dt).unwrap();
        let dt_new: DateTimeNative = rbson::from_bson(s).unwrap();
        println!("{},{}", dt.timestamp_millis(), dt_new.timestamp_millis());
        assert_eq!(dt, dt_new);
    }

    #[test]
    fn test_decode() {
        let s = rbson::Bson::String("2015-09-18T23:56:04".to_string());
        let dt_new: DateTimeNative = rbson::from_bson(s).unwrap();
        println!("{},{}", 1442620564000i64, dt_new.timestamp_millis());
        assert_eq!(1442620564000i64, dt_new.timestamp_millis());
    }

    #[test]
    fn test_decode2() {
        let s = rbson::Bson::String("2015-09-18 23:56:04".to_string());
        let dt_new: DateTimeNative = rbson::from_bson(s).unwrap();
        println!("{},{}", 1442620564000i64, dt_new.timestamp_millis());
        assert_eq!(1442620564000i64, dt_new.timestamp_millis());
    }

    #[test]
    fn test_ser_de() {
        let b = DateTimeNative::now();
        let bsons = rbson::to_bson(&b).unwrap();
        println!("{:?}", bsons);
        let js = serde_json::to_value(&b).unwrap();
        println!("{:?}", js);
    }
}