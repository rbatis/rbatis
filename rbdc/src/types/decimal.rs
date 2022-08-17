use rbs::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Decimal")]
pub struct Decimal(pub String);

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decimal({})", self.0)
    }
}

impl From<Decimal> for Value {
    fn from(arg: Decimal) -> Self {
        Value::Ext("Decimal", Box::new(Value::String(arg.0)))
    }
}

impl FromStr for Decimal{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Decimal(s.to_string()))
    }
}
