use rbs::Value;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq,Hash)]
#[serde(rename = "Time")]
pub struct Time(pub fastdate::Time);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time({})", self.0)
    }
}

impl From<Time> for Value {
    fn from(arg: Time) -> Self {
        Value::Ext("Time", Box::new(Value::String(arg.0.to_string())))
    }
}
