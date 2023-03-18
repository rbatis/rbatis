use crate::{Error, RBDCString};
use rbs::Value;
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Decimal(pub String);

impl RBDCString for Decimal {
    fn ends_name() -> &'static str {
        "DEC"
    }

    fn decode_str(arg: &str) -> Result<Self, crate::Error> {
        let is = Self::is(arg);
        if is != "" {
            return Ok(Self::from_str(arg.trim_end_matches(Self::ends_name()))?);
        }
        Err(crate::Error::E(format!("warn type decode :{}",Self::ends_name())))
    }
}

impl Deref for Decimal{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Decimal{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.to_string();
            s.push_str(Self::ends_name());
            serializer.serialize_str(&s)
        } else {
            self.to_string().serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)? {
            Value::I32(v) => { Ok(Decimal::from(v)) }
            Value::I64(v) => { Ok(Decimal::from(v)) }
            Value::U32(v) => { Ok(Decimal::from(v)) }
            Value::U64(v) => { Ok(Decimal::from(v)) }
            Value::F32(v) => { Ok(Decimal::from(v)) }
            Value::F64(v) => { Ok(Decimal::from(v)) }
            Value::String(mut v) => {
                Decimal::trim_ends_match(&mut v);
                Ok(Decimal::from(v))
            }
            Value::Binary(v) => {
                let s = unsafe { String::from_utf8_unchecked(v) };
                Ok(Decimal::from(s))
            }
            _ => { Err(D::Error::custom("warn type decode Decimal")) }
        }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Decimal> for Value {
    fn from(arg: Decimal) -> Self {
        let mut v = arg.0.to_string();
        v.push_str("DEC");
        Value::String(v)
    }
}

impl From<&str> for Decimal {
    fn from(arg: &str) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<i32> for Decimal {
    fn from(arg: i32) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<i64> for Decimal {
    fn from(arg: i64) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<f32> for Decimal {
    fn from(arg: f32) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<f64> for Decimal {
    fn from(arg: f64) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<u32> for Decimal {
    fn from(arg: u32) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<u64> for Decimal {
    fn from(arg: u64) -> Self {
        Decimal::from(arg.to_string())
    }
}

impl From<String> for Decimal {
    fn from(arg: String) -> Self {
        Decimal(arg)
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal::from(s))
    }
}
