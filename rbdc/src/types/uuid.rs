use rbs::Value;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
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
