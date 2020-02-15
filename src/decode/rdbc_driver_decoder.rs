use rbatis_drivers::Error::General;
use rbatis_drivers::ResultSet;
use serde::de;
use serde::de::DeserializeOwned;
use serde::export::fmt::Error;
use serde::export::Formatter;

use crate::decode::decoder::{is_array, json_len};
use crate::decode::encoder::encode_to_value;
use crate::error::RbatisError;

pub fn decode_result_set<T: ?Sized>(arg: &mut dyn ResultSet) -> (Result<T, RbatisError>, usize)
    where T: DeserializeOwned {
    let mut js = serde_json::Value::Null;
    let type_name = std::any::type_name::<T>();
    let datas = encode_to_value(arg);
    let len = datas.len();
    if is_array::<T>(type_name) {
        //decode array
        js = serde_json::Value::Array(datas);
    } else {
        match std::any::type_name::<T>() {
            "i32" | "u32" | "f32" | "i64" | "u64" | "f64" | "serde_json::number::Number" => {
                //decode number
                let mut size = 0;
                for item in datas {
                    if size > 0 {
                        continue;
                    }
                    match item {
                        serde_json::Value::Object(arg) => {
                            for (_, r) in arg {
                                js = r;
                                break;
                            }
                        }
                        _ => {}
                    }
                    size += 1;
                }
            }
            "serde_json::value::Value" => {
                //decode json
                js = serde_json::Value::Array(datas)
            }
            _ => {
                //decode struct
                let len = datas.len();
                if datas.len() > 1 {
                    return (Result::Err(RbatisError::from(format!("[rbatis] rows.affected_rows > 1,but decode one result({})!", type_name))), len);
                }
                for x in datas {
                    js = x;
                }
            }
        }
    }
    let decode_result = serde_json::from_value(js);
    if decode_result.is_ok() {
        return (Result::Ok(decode_result.unwrap()), len);
    } else {
        let e = decode_result.err().unwrap().to_string();
        return (Result::Err(RbatisError::from("[rbatis] json decode: ".to_string()+type_name+" fail:" + e.as_str())), len);
    }
}