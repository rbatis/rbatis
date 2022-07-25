use std::fmt::{Display, Formatter};
use byteorder::BigEndian;
use rbdc::Error;
use crate::value::PgValueRef;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Bytea")]
pub struct Bytea(u8);

impl Display for Bytea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bytea({})", self.0)
    }
}
