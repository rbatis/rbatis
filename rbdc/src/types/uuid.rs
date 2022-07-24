use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "uuid")]
pub struct Uuid(String);

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "uuid")?;
        write!(f, "{}", self.0)?;
        write!(f, ")")
    }
}