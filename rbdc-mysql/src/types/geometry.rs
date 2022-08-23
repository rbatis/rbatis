use crate::io::MySqlBufMutExt;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;
use rbdc::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
#[serde(rename = "Geometry")]
pub struct Geometry(pub Vec<u8>);

impl Display for Geometry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geometry({})", hex::encode(&self.0))
    }
}

impl Debug for Geometry{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geometry({:?})", self.0)
    }
}

impl Encode for Geometry {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.put_bytes_lenenc(self.0);
        Ok(buf.len())
    }
}

impl Decode for Geometry {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Geometry(value.as_bytes().unwrap_or_default().to_vec()))
    }
}
