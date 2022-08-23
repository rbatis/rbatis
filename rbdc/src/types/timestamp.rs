use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Timestamp")]
pub struct Timestamp(pub u64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamp({})", self.0)
    }
}

impl Debug for Timestamp{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timestamp({})", self.0)
    }
}

impl From<Timestamp> for Value {
    fn from(arg: Timestamp) -> Self {
        Value::Ext("Timestamp", Box::new(Value::U64(arg.0)))
    }
}

impl FromStr for Timestamp{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Timestamp(u64::from_str(s)?))
    }
}
