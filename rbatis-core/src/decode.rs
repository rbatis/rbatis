//! Types and traits for decoding values from the database.
use serde::de::DeserializeOwned;

use crate::Error;

/// decode json vec to an object
/// support decode types:
/// serde_json::Value,BigDecimal, i8..i64,u8..u64,serde_json::Number,bool,String
/// or object used serde_json macro object
pub fn json_decode<T: ?Sized>(datas: Vec<serde_json::Value>) -> Result<T, crate::Error>
    where T: DeserializeOwned {
    let mut js = serde_json::Value::Null;
    let type_name = std::any::type_name::<T>();
    if is_array(type_name) {
        //decode array
        js = serde_json::Value::Array(datas);
    } else {
        match type_name {
            //decode single type option
            "core::option::Option<i8>" | "core::option::Option<i16>" | "core::option::Option<i32>" | "core::option::Option<i64>" |
            "core::option::Option<u8>" | "core::option::Option<u16>" | "core::option::Option<u32>" | "core::option::Option<u64>" |
            "core::option::Option<f32>" | "core::option::Option<f64>" |
            "core::option::Option<serde_json::number::Number>" |
            "core::option::Option<bigdecimal::BigDecimal>" |
            "core::option::Option<bool>" |
            "core::option::Option<alloc::string::String>" |
            //decode single type(from map type get an value)
            "i8" | "i16" | "i32" | "i64" |
            "u8" | "u16" | "u32" | "u64" |
            "f32" | "f64" |
            "serde_json::number::Number" |
            "bigdecimal::BigDecimal" |
            "bool" |
            "alloc::string::String" => {
                //decode struct
                if datas.len() > 1 {
                    return Result::Err(Error::from(format!("[rbatis] rows.rows_affected > 1,but decode one result({})!", type_name)));
                }
                for item in datas {
                    match item {
                        serde_json::Value::Object(arg) => {
                            for (_, r) in arg {
                                match type_name {
                                    "alloc::string::String" | "bigdecimal::BigDecimal" => {
                                        js = serde_json::Value::String(r.to_string());
                                    }
                                    _ => {
                                        js = r;
                                    }
                                }
                                break;
                            }
                        }
                        _ => {}
                    }
                    break;
                }
            }
            "serde_json::value::Value" => {
                //decode json
                js = serde_json::Value::Array(datas)
            }
            _ => {
                //decode struct
                if datas.len() > 1 {
                    return Result::Err(Error::from(format!("[rbatis] rows.rows_affected > 1,but decode one result({})!", type_name)));
                }
                //decode single object
                for x in datas {
                    js = x;
                    break;
                }
            }
        }
    }

    //debug_mode feature print json_decode json data
    #[cfg(feature = "debug_mode")]
    println!("[rbatis] [debug_mode] => {:#}", js);

    let decode_result = serde_json::from_value(js);
    if decode_result.is_ok() {
        return Result::Ok(decode_result.unwrap());
    } else {
        let e = decode_result.err().unwrap().to_string();
        return Result::Err(Error::from(format!("[rbatis] json_decode fail decode_type:\"{}\",serde_err:\"{}\"", type_name, e)));
    }
}

fn is_array(type_name: &str) -> bool {
    if type_name.starts_with("[")
        || type_name.starts_with("&[")
        || type_name.starts_with("alloc::collections::linked_list")
        || type_name.starts_with("alloc::collections::vec_deque")
        || type_name.starts_with("std::collections::hash::set")
        || type_name.starts_with("alloc::vec::Vec<")

        || type_name.starts_with("core::option::Option<[")
        || type_name.starts_with("core::option::Option<&[")
        || type_name.starts_with("core::option::Option<alloc::vec::Vec<")
        || type_name.starts_with("core::option::Option<alloc::collections::linked_list")
        || type_name.starts_with("core::option::Option<alloc::collections::vec_deque")
        || type_name.starts_with("core::option::Option<std::collections::hash::set")
    {
        return true;
    }
    return false;
}


#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

    use serde_json::json;

    use crate::decode::{is_array, json_decode};

    #[test]
    fn test_decode_hashmap() {
        let m: HashMap<String, serde_json::Value> = json_decode(vec![json!(
        {
        "a":"1",
        "b":2
        }
        )]).unwrap();
        println!("{:#?}", m);
    }

    #[test]
    fn test_decode_btree_map() {
        let m: BTreeMap<String, serde_json::Value> = json_decode(vec![json!(
        {
        "a":"1",
        "b":2
        }
        )]).unwrap();
        println!("{:#?}", m);
    }

    #[test]
    fn test_is_array() {
        let dq_name = std::any::type_name::<VecDeque::<i32>>();
        assert_eq!(is_array(dq_name), true);
    }
}