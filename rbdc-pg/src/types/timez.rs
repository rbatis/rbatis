use std::fmt::{Display, Formatter};
use rbdc::Error;
use rbs::Value;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::PgValue;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Timez")]
pub struct Timetz(pub fastdate::Time);

impl Display for Timetz {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timetz({})", self.0)
    }
}

impl From<Timetz> for Value {
    fn from(arg: Timetz) -> Self {
        Value::Ext("Timetz", Box::new(Value::String(arg.0.to_string())))
    }
}

impl Decode for Timetz{
    fn decode(value: PgValue) -> Result<Self, Error> {
        todo!()
    }
}

impl Encode for Timetz{
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        todo!()
    }
}