use crate::{Error};
use rbs::{to_value, Value};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use serde::Serializer;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Time(pub fastdate::Time);

impl serde::Serialize for Time{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if std::any::type_name::<S>() == std::any::type_name::<rbs::Serializer>() {
            let mut s = self.0.to_string();
            s.push_str("T");
            serializer.serialize_str(&s)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if std::any::type_name::<D>() == std::any::type_name::<rbs::Serializer>() {
            use serde::de::Error;
            let mut value = Value::deserialize(deserializer)?;
            match &mut value {
                Value::String(v) => {
                    if v.ends_with("T"){
                        v.pop();
                    }
                }
                _ => {}
            }
            let time: fastdate::Time = rbs::from_value(value)
                .map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
            Ok(Time::from(time))
        } else {
            use serde::de::Error;
            let time: fastdate::Time = rbs::from_value(Value::deserialize(deserializer)?)
                .map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
            Ok(Time::from(time))
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Time> for Value {
    fn from(arg: Time) -> Self {
        to_value!(arg)
    }
}

impl From<fastdate::Time> for Time {
    fn from(value: fastdate::Time) -> Self {
        Self(value)
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time::from(fastdate::Time::from_str(s)?))
    }
}
