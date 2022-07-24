use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "decimal")]
pub struct Decimal(String);

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "decimal")?;
        write!(f, "{}", self.0)?;
        write!(f, ")")
    }
}


