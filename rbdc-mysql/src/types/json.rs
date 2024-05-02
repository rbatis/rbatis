use crate::io::MySqlBufMutExt;
use crate::types::{Decode, Encode};
use crate::value::MySqlValue;
use rbdc::json::Json;
use rbdc::Error;

impl Encode for Json {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let mut bytes = self.0.into_bytes();
        if bytes.is_empty() {
            bytes = "null".to_string().into_bytes();
        }
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

pub fn encode_json(arg: rbs::Value, buf: &mut Vec<u8>) -> Result<usize, Error> {
    let bytes = arg.to_string().into_bytes();
    let len = bytes.len();
    buf.put_bytes_lenenc(bytes);
    Ok(len)
}

pub fn decode_json(value: MySqlValue) -> Result<rbs::Value, Error> {
    let v = value.as_str().unwrap_or("null").to_string();
    Ok(serde_json::from_str(&v).map_err(|e| Error::from(e.to_string()))?)
}
