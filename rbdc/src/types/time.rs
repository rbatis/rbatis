use std::borrow::Cow;
use crate::{Error, IntoValue};
use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Time")]
pub struct Time {
    pub r#type: Cow<'static,str>,
    pub value: fastdate::Time,
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let time: fastdate::Time = rbs::from_value(Value::deserialize(deserializer)?.into_value())
            .map_err(|e| D::Error::custom(&format!("warn type decode Time:{}",e)))?;
        Ok(Time::from(time))
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Time> for Value {
    fn from(arg: Time) -> Self {
        Value::from(vec![("Time".into(), Value::String(arg.value.to_string()))])
    }
}

impl From<fastdate::Time> for Time {
    fn from(value: fastdate::Time) -> Self {
        Self {
            r#type: Cow::Borrowed("Time"),
            value,
        }
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time::from(fastdate::Time::from_str(s)?))
    }
}
