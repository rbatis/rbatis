use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "DateTime")]
pub struct DateTimeStr(String);

impl Display for DateTimeStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Deref for DateTimeStr {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTimeStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTime(fastdate::DateTime);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DateTime({})", self.0)
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("DateTime", &self.0)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let v = DateTimeStr::deserialize(deserializer)?;
        Ok(Self(fastdate::DateTime::from_str(&v.0).map_err(|e| D::Error::custom(e.to_string()))?))
    }
}

impl Deref for DateTime {
    type Target = fastdate::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self(fastdate::DateTime::now())
    }
    pub fn utc() -> Self {
        Self(fastdate::DateTime::utc())
    }
}

#[test]
fn test() {
    let date = DateTimeStr("2017-02-06T00-00-00".to_string());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
    let date = DateTime(fastdate::DateTime::now());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
}
