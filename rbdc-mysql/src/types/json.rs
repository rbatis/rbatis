use crate::io::MySqlBufMutExt;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;
use rbdc::json::Json;
use rbdc::Error;

impl Encode for Json {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let mut bytes = self.0.into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}
impl Decode for Json {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self(value.as_str().unwrap_or("null").to_string()))
    }
}
