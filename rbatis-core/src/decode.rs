//! Types and traits for decoding values from the database.
use serde::de;
use serde::de::DeserializeOwned;
use serde::export::fmt::Error;
use serde::export::Formatter;

use crate::database::Database;
use crate::value::HasRawValue;

/// A type that can be decoded from the database.
pub trait Decode<'de, DB>
    where
        Self: Sized + 'de,
        DB: Database,
{
    fn decode(value: <DB as HasRawValue<'de>>::RawValue) -> crate::Result<Self>;
}

/// decode json vec to an object
pub fn json_decode<T: ?Sized>(datas: Vec<serde_json::Value>) -> Result<T, crate::Error>
    where T: DeserializeOwned {
    let mut js = serde_json::Value::Null;
    let type_name = std::any::type_name::<T>();
    if is_array::<T>(type_name) {
        //decode array
        js = serde_json::Value::Array(datas);
    } else {
        match type_name {
            //decode single type(from map type get an value)
            "i8" | "i16" | "i32" | "i64" |
            "u8" | "u16" | "u32" | "u64" |
            "f32" | "f64" |
            "serde_json::number::Number" |
            "bigdecimal::BigDecimal" |
            "bool" => {
                //decode struct
                if datas.len() > 1 {
                    return Result::Err(decode_err!("[rbatis] rows.affected_rows > 1,but decode one result({})!", type_name));
                }
                for item in datas {
                    match item {
                        serde_json::Value::Object(arg) => {
                            for (_, r) in arg {
                                js = r;
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
            "alloc::string::String" => {
                js = serde_json::Value::Array(datas);
                js = serde_json::Value::String(js.to_string());
            }
            _ => {
                //decode struct
                if datas.len() > 1 {
                    return Result::Err(decode_err!("[rbatis] rows.affected_rows > 1,but decode one result({})!", type_name));
                }
                //decode single object
                for x in datas {
                    js = x;
                    break;
                }
            }
        }
    }
    let decode_result = serde_json::from_value(js);
    if decode_result.is_ok() {
        return Result::Ok(decode_result.unwrap());
    } else {
        let e = decode_result.err().unwrap().to_string();
        return Result::Err(decode_err!("[rbatis] json decode: {}, fail:{}" ,type_name, e));
    }
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