use crate::{Error, IntoValue};
use rbs::Value;
use serde::Deserializer;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Decimal")]
pub struct Decimal {
    pub r#type: &'static str,
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)?.into_value().into_string() {
            None => Err(D::Error::custom("warn type decode Decimal")),
            Some(v) => Ok(Self::from(v)),
        }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Decimal> for Value {
    fn from(arg: Decimal) -> Self {
        Value::Map(rbs::value::map::ValueMap{
            inner: vec![("type".into(),"Decimal".into()),("value".into(),arg.value.into())],
        })
    }
}

impl From<&str> for Decimal {
    fn from(arg: &str) -> Self {
        Decimal::from(arg.to_string())
    }
}
impl From<String> for Decimal {
    fn from(arg: String) -> Self {
        Decimal {
            r#type: "Decimal",
            value: arg,
        }
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal::from(s))
    }
}
