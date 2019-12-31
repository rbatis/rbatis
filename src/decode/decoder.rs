use serde::de;
use rbatis_macro::RbatisMacro;

pub type Error = String;

pub trait Decoder {
    fn decode<T>(&mut self) -> Result<T, Error>
        where
            T: de::DeserializeOwned + RbatisMacro;
}

