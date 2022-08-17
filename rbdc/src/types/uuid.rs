use rbs::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Uuid")]
pub struct Uuid(pub String);

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uuid({})", self.0)
    }
}

impl From<Uuid> for Value {
    fn from(arg: Uuid) -> Self {
        Value::Ext("Uuid", Box::new(Value::String(arg.0)))
    }
}

impl FromStr for Uuid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Uuid(s.to_string()))
    }
}