use crate::{Error, IntoValue};
use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Time")]
pub struct Time(pub fastdate::Time);

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let time:fastdate::Time=rbs::from_value(Value::deserialize(deserializer)?.into_value()).map_err(|_|D::Error::custom("warn type decode Date"))?;
        Ok(Time(time))
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
        Value::from(("Time",Value::String(arg.0.to_string())))
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time(fastdate::Time::from_str(s)?))
    }
}
