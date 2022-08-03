use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use rbs::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FastDateTime(pub fastdate::DateTime);

impl Display for FastDateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Serialize for FastDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("DateTime", &self.0)
    }
}

impl<'de> Deserialize<'de> for FastDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let v = DateTime::deserialize(deserializer)?;
        Ok(Self(fastdate::DateTime::from_str(&v.0).map_err(|e| D::Error::custom(e.to_string()))?))
    }
}

impl Deref for FastDateTime {
    type Target = fastdate::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FastDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FastDateTime {
    pub fn now() -> Self {
        Self(fastdate::DateTime::now())
    }
    pub fn utc() -> Self {
        Self(fastdate::DateTime::utc())
    }
}

impl From<FastDateTime> for Value {
    fn from(arg: FastDateTime) -> Self {
        Value::Ext("DateTime", Box::new(Value::String(arg.0.to_string())))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "DateTime")]
pub struct DateTime(pub String);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Deref for DateTime {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test() {
    let date = DateTime("2017-02-06T00-00-00".to_string());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
    let date = FastDateTime(fastdate::DateTime::now());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
}
