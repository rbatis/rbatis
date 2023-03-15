use crate::io::MySqlBufMutExt;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;
use rbdc::json::Json;
use rbdc::Error;

impl Encode for Json {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let bytes = self.value.to_string().into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}
impl Decode for Json {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        let js=serde_json::from_str(value.as_str().unwrap_or("null")).unwrap_or(serde_json::Value::Null);
        Ok(Self::from(js))
    }
}
