use std::fmt::{Display, Formatter};
use rbs::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Decimal")]
pub struct Decimal(pub String);

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decimal({})", self.0)
    }
}

impl From<Decimal> for Value{
    fn from(arg: Decimal) -> Self {
        Value::Ext("Decimal",Box::new(Value::String(arg.0)))
    }
}
