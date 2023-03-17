use crate::{Error, IntoValue};
use rbs::Value;
use serde::Deserializer;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(serde::Serialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Uuid")]
pub struct Uuid {
    pub r#type: &'static str,
    pub value: String,
}

impl<'de> serde::Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        match Value::deserialize(deserializer)?.into_value().into_string() {
            None => Err(D::Error::custom("warn type decode Uuid")),
            Some(v) => Ok(Self::from(v)),
        }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for Value {
    fn from(arg: Uuid) -> Self {
        Value::from(vec![("Uuid".into(), Value::String(arg.value))])
    }
}

impl From<&str> for Uuid {
    fn from(arg: &str) -> Self {
        Uuid::from(arg.to_string())
    }
}

impl From<String> for Uuid {
    fn from(arg: String) -> Self {
        Uuid {
            r#type: "Uuid",
            value: arg,
        }
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
