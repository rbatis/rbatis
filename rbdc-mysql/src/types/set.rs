use crate::io::MySqlBufMutExt;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;
use rbdc::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
#[serde(rename = "Set")]
pub struct Set(pub String);

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Set({})", self.0)
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Set({})", self.0)
    }
}

impl Encode for Set {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let bytes = self.0.into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}

impl Decode for Set {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self(value.as_str().unwrap_or_default().to_string()))
    }
}
