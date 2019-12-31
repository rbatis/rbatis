use serde::de;

pub type Error = String;

pub trait Decoder {
    fn decode<T:?Sized>(&mut self) -> Result<T, Error>
        where
            T: de::DeserializeOwned;
}

pub fn is_array<T:?Sized>(type_name:&str) -> bool
    where
        T: de::DeserializeOwned{
    if type_name.starts_with("alloc::collections::linked_list")
        ||type_name.starts_with("alloc::vec::Vec<")
        || type_name.starts_with("[")
        || type_name.starts_with("&["){
        return true;
    }
    return false;
}

