use rbs::Value;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Json")]
pub struct Json(pub String);

impl Default for Json {
    fn default() -> Self {
        Self {
            0: "null".to_string(),
        }
    }
}
impl From<serde_json::Value> for Json {
    fn from(arg: serde_json::Value) -> Self {
        Json(arg.to_string())
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Json({})", self.0)
    }
}

impl From<Json> for Value {
    fn from(arg: Json) -> Self {
        Value::Ext("Json", Box::new(Value::String(arg.0)))
    }
}

impl FromStr for Json{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
