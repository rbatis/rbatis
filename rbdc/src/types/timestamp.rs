
#[derive(serde::Serialize,serde::Deserialize,Debug,Clone,Eq, PartialEq)]
#[serde(rename = "timestamp")]
pub struct Timestamp(u64);

