use crate::decode::Decoder::JsonDecoder;
use postgres::rows::Rows;
use serde::de;
use rbatis_macro::RbatisMacro;

//PG 解析器
impl JsonDecoder for Rows{
    fn decode_json<T>(&mut self) -> Result<T, String> where
        T: de::DeserializeOwned + RbatisMacro {
        unimplemented!()
    }
}