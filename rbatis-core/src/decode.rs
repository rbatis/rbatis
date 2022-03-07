//! Types and traits for decoding values from the database.
use rbson::Bson;
use log::info;
use serde::de::DeserializeOwned;

use crate::{Error};
use crate::Error::E;
use crate::types::Format;

/// decode json vec to an object
/// support decode types:
/// Bson,BigDecimal, i8..i64,u8..u64,rbson::Int64(),bool,String
/// or object used bson macro object
pub fn decode<T: ?Sized>(mut datas: rbson::Array) -> Result<T, crate::Error>
    where
        T: DeserializeOwned,
{
    let mut type_name = std::any::type_name::<T>();
    //debug_mode feature print json_decode json data
    let bs = Bson::Array(datas);
    #[cfg(feature = "debug_mode")]
        {
            println!("[rbatis] [debug_mode] [bson]   {} => {}", type_name, bs);
            #[cfg(feature = "format_bson")]
            println!("[rbatis] [debug_mode] [format] {} => {}", type_name, bs.as_array().unwrap().do_format());
        }
    let mut datas = vec![];
    match bs {
        Bson::Array(arr) => {
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
        "core::option::Option<rbson::Bson::Int64>" | "core::option::Option<rbson::Bson::Int32>" | "core::option::Option<rbson::Bson::Double>" |
        "core::option::Option<bigdecimal::BigDecimal>" |
        "core::option::Option<bool>" |
        "core::option::Option<alloc::string::String>" |
        //decode single type(from map type get an value)
        "i8" | "i16" | "i32" | "i64" |
        "u8" | "u16" | "u32" | "u64" |
        "f32" | "f64" |
        "serde_json::number::Number" |
        "rbson::Bson::Int64" | "rbson::Bson::Int32" | "rbson::Bson::Double" |
        "bigdecimal::BigDecimal" |
        "bool" |
        "alloc::string::String" => {
            return Ok(try_decode_doc(type_name, &mut datas)?);
        }
        _ => {}
    }
    // try speculate type
    let is_array: Result<T, rbson::de::Error> = rbson::from_bson(rbson::Bson::Array(vec![]));
    if is_array.is_ok() {
        //decode array
        Ok(rbson::from_bson(Bson::Array(datas))?)
    } else {
        Ok(try_decode_doc(type_name, &mut datas)?)
    }
}

//decode doc or one type
pub fn try_decode_doc<T>(type_name: &str, datas: &mut Vec<Bson>) -> Result<T, crate::Error>
    where
        T: DeserializeOwned, {
    //decode struct
    if datas.len() > 1 {
        return Result::Err(Error::from(format!("[rbatis] rows.rows_affected > 1,but decode one type ({})!", type_name)));
    }
    //single try decode
    if datas.is_empty() {
        return Ok(rbson::from_bson::<T>(Bson::Null)?);
    }
    let mut v = Bson::Null;
    let m = datas.remove(0);
    let mut doc_len = 0;
    match &m {
        Bson::Document(doc) => {
            doc_len = doc.len();
            if doc_len == 1 {
                for (k, _v) in doc {
                    v = _v.clone();
                    break;
                }
            }
        }
        _ => {}
    }
    let r = rbson::from_bson::<T>(m);
    if r.is_err() {
        if doc_len > 1 {
            return Ok(r?);
        }
        //try one
        return Ok(rbson::from_bson::<T>(v)?);
    } else {
        return Ok(r.unwrap());
    }
}


#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
    use rbson::Bson;
    use rbson::bson;
    use crate::decode::{decode};
    use crate::types::Json;

    #[test]
    fn test_decode_hashmap() {
        let m: HashMap<String, Bson> = decode(vec![bson!(
        {
        "a":"1",
        "b":2
        }
        )])
            .unwrap();
        println!("{:#?}", m);
    }

    #[test]
    fn test_decode_btree_map() {
        let m: BTreeMap<String, Bson> = decode(vec![bson!(
        {
        "a":"1",
        "b":2
        }
        )])
            .unwrap();
        println!("{:#?}", m);
    }
}
