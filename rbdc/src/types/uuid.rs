use crate::{Error, RBDCString};
use rbs::{to_value, Value};
use serde::{Deserializer, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive( Clone, Eq, PartialEq, Hash)]
pub struct Uuid(pub String);

impl RBDCString for Uuid {
    fn ends_name() -> &'static str {
        "TS"
    }

    fn decode_str(arg: &str) -> Result<Self, crate::Error> {
        let is = Self::is(arg);
        if is != "" {
            return Ok(Self::from_str(arg.trim_end_matches(Self::ends_name()))?);
        }
        Err(Error::E(format!("warn type decode :{}",Self::ends_name())))
    }
}

impl Deref for Uuid{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Uuid {
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
impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)? {
            Value::String(mut v) => {
                Uuid::trim_ends_match(&mut v);
                Ok(Uuid::from(v))
            }
            Value::Binary(v) => {
                let s = unsafe { String::from_utf8_unchecked(v) };
                Ok(Uuid::from(s))
            }
            _ => { Err(D::Error::custom("warn type decode Decimal")) }
        }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for Value {
    fn from(arg: Uuid) -> Self {
        to_value!(arg)
    }
}

impl From<&str> for Uuid {
    fn from(arg: &str) -> Self {
        Uuid::from(arg.to_string())
    }
}

impl From<String> for Uuid {
    fn from(arg: String) -> Self {
        Uuid(arg)
    }
}

impl FromStr for Uuid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uuid::from(s))
    }
}

impl Uuid {
    ///new for uuid v4
    pub fn new() -> Self {
        Self::from(uuid::Uuid::new_v4().to_string())
    }
}
