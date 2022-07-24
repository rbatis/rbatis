use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "datetime")]
pub struct DateTime(String);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "datetime({})", self.0)
    }
}

impl Deref for DateTime{
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DateTimeFastDate(fastdate::DateTime);

impl Display for DateTimeFastDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "datetime({})", self.0)
    }
}

impl Serialize for DateTimeFastDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_newtype_struct("datetime", &self.0)
    }
}

impl<'de> Deserialize<'de> for DateTimeFastDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let v = DateTime::deserialize(deserializer)?;
        Ok(Self(fastdate::DateTime::from_str(&v.0).map_err(|e| D::Error::custom(e.to_string()))?))
    }
}

impl Deref for DateTimeFastDate{
    type Target = fastdate::DateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTimeFastDate{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[test]
fn test() {
    let date = DateTime("2017-02-06T00-00-00".to_string());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
    let date = DateTimeFastDate(fastdate::DateTime::now());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
}
