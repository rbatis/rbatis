use serde_json::Value;

#[derive(serde::Serialize,serde::Deserialize,Debug,Clone,Eq, PartialEq)]
#[serde(rename = "json")]
pub struct Json(String);

impl From<serde_json::Value> for Json{
    fn from(arg: Value) -> Self {
       Json(arg.to_string())
    }
}

