use crate::decode::Decoder::Decoder;
use postgres::rows::Rows;
use serde::de;
use rbatis_macro::RbatisMacro;

//PG 解析器
impl Decoder for Rows{
    fn decode<T>(&mut self) -> Result<T, String> where
        T: de::DeserializeOwned + RbatisMacro {
        unimplemented!()
    }
}