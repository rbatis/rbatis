use serde::de;
use rbatis_macro::RbatisMacro;

pub type Error = String;

pub trait Decoder {
    fn decode<T>(&mut self) -> Result<T, Error>
        where
            T: de::DeserializeOwned + RbatisMacro;
}

pub fn is_json_array_type(decode_name: &str) -> bool {
    if decode_name == "Vec" || decode_name == "Array" || decode_name == "Slice" || decode_name == "LinkedList" {
        return true;
    }
    return false;
}

pub fn is_number_type(decode_name: &str) -> bool {
    if decode_name == "i32" || decode_name == "u32" || decode_name == "f32" || decode_name == "i64" || decode_name == "u64" || decode_name == "f64" {
        return true;
    }
    return false;
}