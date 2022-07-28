use std::fmt::{Display, Formatter};
use rbs::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Date")]
pub struct Date(pub fastdate::Date);

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Date({})", self.0)
    }
}

impl From<Date> for Value {
    fn from(arg: Date) -> Self {
        Value::Ext("Date", Box::new(Value::String(arg.0.to_string())))
    }
}