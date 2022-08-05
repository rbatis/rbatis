use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat, PgValueRef};
use byteorder::BigEndian;
use rbdc::Error;
use rbs::Value;
use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Bytea")]
pub struct Bytea(pub u8);

impl Display for Bytea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "bytea({})", self.0)
    }
}

impl Encode for Bytea {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.push(self.0);
        Ok(IsNull::No)
    }
}

impl Decode for Bytea {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(Self(
            value.as_bytes()?.get(0).copied().unwrap_or_default() as u8
        ))
    }
}

impl From<Bytea> for Value {
    fn from(arg: Bytea) -> Self {
        Value::Ext("Bytea", Box::new(Value::U32(arg.0 as u32)))
    }
}

impl Encode for Vec<u8> {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(self);
        Ok(IsNull::No)
    }
}

impl Decode for Vec<u8> {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => value.into_bytes(),
            PgValueFormat::Text => {
                Err("unsupported decode to `&[u8]` of BYTEA in a simple query; use a prepared query or decode to `Vec<u8>`".into())
            }
        }
    }
}
