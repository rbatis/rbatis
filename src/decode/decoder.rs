use serde::de;

pub type Error = String;

pub trait Decoder {
    fn decode<T: ?Sized>(&mut self, decode_len: &mut usize) -> Result<T, Error>
        where
            T: de::DeserializeOwned;
}


pub fn is_array<T: ?Sized>(type_name: &str) -> bool
    where
        T: de::DeserializeOwned {
    if type_name.starts_with("alloc::collections::linked_list")
        || type_name.starts_with("alloc::vec::Vec<")
        || type_name.starts_with("[")
        || type_name.starts_with("&[") {
        return true;
    }
    return false;
}

pub fn json_len(js: &serde_json::Value) -> usize {
    if js.is_null() {
        return 0;
    } else if js.is_array() {
        return js.as_array().unwrap().len();
    } else {
        return 1;
    }
}
