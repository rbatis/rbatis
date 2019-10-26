use serde::de;
use rbatis_macro::RbatisMacro;

pub type Error = String;

pub trait Decoder{
     fn decode<T>(&mut self) -> Result<T, Error>
          where
              T: de::DeserializeOwned + RbatisMacro;
}

pub fn isJsonArrayType(decode_name:&str)->bool{
     if decode_name == "Vec" || decode_name == "Array" || decode_name == "Slice" || decode_name == "LinkedList" {
          return true;
     }
     return false;
}