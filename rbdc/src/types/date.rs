use crate::{Error, IntoValue};
use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Date")]
pub struct Date(pub fastdate::Date);

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let date:fastdate::Date=rbs::from_value(Value::deserialize(deserializer)?.into_value()).map_err(|_|D::Error::custom("warn type decode Date"))?;
        Ok(Date(date))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Date> for Value {
    fn from(arg: Date) -> Self {
        Value::from(("Date",Value::String(arg.0.to_string())))
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date(fastdate::Date::from_str(s)?))
    }
}
