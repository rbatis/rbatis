use crate::{Error, IntoValue};
use rbs::{to_value, Value};
use serde::Deserializer;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Date")]
pub struct Date {
    pub r#type: &'static str,
    pub value: fastdate::Date,
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let time: fastdate::Date = rbs::from_value(Value::deserialize(deserializer)?.into_value())
            .map_err(|e| D::Error::custom(&format!("warn type decode Date:{}", e)))?;
        Ok(Date::from(time))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Date> for Value {
    fn from(arg: Date) -> Self {
        Value::Map(rbs::value::map::ValueMap{
            inner: vec![("type".into(),"Date".into()),("value".into(), to_value!(arg.value))],
        })
    }
}

impl From<fastdate::Date> for Date {
    fn from(arg: fastdate::Date) -> Self {
        Date {
            r#type: "Date",
            value: arg,
        }
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date::from(fastdate::Date::from_str(s)?))
    }
}
