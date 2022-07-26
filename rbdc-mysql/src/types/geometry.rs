use std::fmt::{Display, Formatter};
use rbdc::Error;
use crate::io::MySqlBufMutExt;
use crate::result_set::MySqlTypeInfo;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "Geometry")]
pub struct Geometry(pub Vec<u8>);

impl Display for Geometry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geometry({})", hex::encode(&self.0))
    }
}

impl Encode for Geometry{
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.put_bytes_lenenc(self.0);
        Ok(buf.len())
    }
}

impl Decode for Geometry{
    fn decode(value: MySqlValue) -> Result<Self, Error> {
       Ok(Geometry(value.as_bytes().unwrap_or_default().to_vec()))
    }
}