//! Types and traits for decoding values from the database.
use log::info;
use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// Value,BigDecimal, i8..i64,u8..u64,Int64(),bool,String
/// or object used bson macro object
pub fn decode<T: ?Sized>(bs: Value) -> Result<T, crate::Error>
where
    T: DeserializeOwned,
{
    let type_name = std::any::type_name::<T>();
    //debug_mode feature print json_decode json data
    #[cfg(feature = "debug_mode")]
    {
        println!("[rbatis] [debug_mode] [value]   {} => {}", type_name, bs);
    }
    let mut datas = vec![];
    match bs {
        Value::Array(arr) => {
            datas = arr;
        }
        _ => {}
    }
    // Hit a non-array object
    match type_name {
        //decode single type option
        "core::option::Option<i8>" | "core::option::Option<i16>" | "core::option::Option<i32>" | "core::option::Option<i64>" |
        "core::option::Option<u8>" | "core::option::Option<u16>" | "core::option::Option<u32>" | "core::option::Option<u64>" |
        "core::option::Option<f32>" | "core::option::Option<f64>" |
        "core::option::Option<serde_json::number::Number>" |
        "core::option::Option<rbs::Value::I64>" | "core::option::Option<rbs::Value::I32>" | "core::option::Option<rbs::Value::F64>" | "core::option::Option<rbs::Value::F32>" |
        "core::option::Option<bigdecimal::BigDecimal>" |
        "core::option::Option<bool>" |
        "core::option::Option<alloc::string::String>" |
        //decode single type(from map type get an value)
        "i8" | "i16" | "i32" | "i64" |
        "u8" | "u16" | "u32" | "u64" |
        "f32" | "f64" |
        "serde_json::number::Number" |
        "rbs::Value::I64" | "rbs::Value::I32" | "rbs::Value::F64" | "rbs::Value::F32" |
        "bigdecimal::BigDecimal" |
        "bool" |
        "alloc::string::String" => {
            return Ok(try_decode_map(type_name, &mut datas)?);
        }
        _ => {}
    }
    // try speculate type
    let is_array: Result<T, rbs::Error> = rbs::from_value(Value::Array(vec![]));
    if is_array.is_ok() {
        //decode array
        Ok(rbs::from_value(Value::Array(datas))?)
    } else {
        Ok(try_decode_map(type_name, &mut datas)?)
    }
}

//decode doc or one type
pub fn try_decode_map<T>(type_name: &str, datas: &mut Vec<Value>) -> Result<T, crate::Error>
where
    T: DeserializeOwned,
{
    //decode struct
    if datas.len() > 1 {
        return Result::Err(Error::from(format!(
            "[rbatis] rows.rows_affected > 1,but decode one type ({})!",
            type_name
        )));
    }
    //single try decode
    if datas.is_empty() {
        return Ok(rbs::from_value::<T>(Value::Null)?);
    }
    let mut v = Value::Null;
    let m = datas.remove(0);
    let mut doc_len = 0;
    match &m {
        Value::Map(doc) => {
            doc_len = doc.len();
            if doc_len == 1 {
                if let Some((_, value)) = doc.into_iter().next() {
                    v = value.clone();
                }
            }
        }
        _ => {}
    }
    let r = rbs::from_value::<T>(m);
    if r.is_err() {
        if doc_len > 1 {
            return Ok(r?);
        }
        //try one
        return Ok(rbs::from_value::<T>(v)?);
    } else {
        return Ok(r.unwrap());
    }
}

#[cfg(test)]
mod test {
    use rbs::{to_value, Value};
    use std::collections::HashMap;
    use rbs::value::map::ValueMap;
    use crate::decode::decode;

    #[test]
    fn test_decode_hashmap() {
        let mut v = ValueMap::new();
        v.insert(1.into(),2.into());
        let m: HashMap<i32, Value> = decode(Value::Array(vec![Value::Map(v)])).unwrap();
        println!("{:#?}", m);
        assert_eq!(m.get(&1).unwrap().as_i64(),Value::I32(2).as_i64());
    }
}
