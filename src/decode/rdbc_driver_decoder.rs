use crate::decode::decoder::{ is_array, json_len};
use rdbc::ResultSet;
use serde::de;
use crate::decode::encoder::encode_to_value;
use serde::de::DeserializeOwned;
use rdbc::Error::General;
use serde::export::Formatter;
use serde::export::fmt::Error;



pub fn decode_result_set<T:?Sized>(arg:&mut ResultSet) -> (Result<T, String>,usize)
        where T: DeserializeOwned {
        let mut js = serde_json::Value::Null;
        let type_name=std::any::type_name::<T>();

        let mut datas=encode_to_value(arg);

        if is_array::<T>(type_name) {
            //decode array
            if datas.len()!=0{
                js = serde_json::Value::Array(datas);
            }
        }else{
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
                                    break
                                }
                            }
                            _ => {}
                        }
                        size += 1;
                    }
                },
                "serde_json::value::Value" => {
                    //decode json
                    js = serde_json::Value::Array(datas)
                },
                _ => {
                    //decode struct
                    let mut index = 0;
                    for item in datas{
                        if index > 1 {
                            return (Result::Err("[rbatis] rows.affected_rows > 1,but decode one result!".to_string()),index);
                        }
                        js = item;
                        index = index + 1;
                    };
                }
            }
        }
        println!("json:{}",js.clone().to_string());
        let len=json_len(&js);
        let decode_result = serde_json::from_value(js);
        if decode_result.is_ok() {
            return (Result::Ok(decode_result.unwrap()),len);
        } else {
            let e = decode_result.err().unwrap().to_string();
            return (Result::Err("[rbatis] json decode fail:".to_string()+e.as_str()),len);
        }
    }