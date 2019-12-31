use crate::decode::decoder::{Decoder, is_array};
use serde::de;
use std::borrow::BorrowMut;
use serde_json::Value;
use postgres::Row;
use std::str::FromStr;
use serde_json::json;
use serde_json::value::Value::Number;

//PG 解析器
impl Decoder for Vec<Row> {
    fn decode<T: ?Sized>(&mut self) -> Result<T, String> where
        T: de::DeserializeOwned {
        let mut js = serde_json::Value::Null;

        let type_name = std::any::type_name::<T>();
        if is_array::<T>(type_name) {
            //decode array
            let mut vec_v = vec![];
            for item in self {
                let act = decode_row(item);
                vec_v.push(act);
            }
            js = serde_json::Value::Array(vec_v);
        } else {
            match std::any::type_name::<T>() {
                "i32" | "u32" | "f32" | "i64" | "u64" | "f64" => {
                    //decode number
                    let mut size = 0;
                    for item in self {
                        if size > 0 {
                            continue;
                        }
                        let act = decode_row(item);
                        match act {
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
                "serde_json::Value" => {
                    //decode json
                    let mut vec_v = vec![];
                    for item in self {
                        let act = decode_row(item);
                        vec_v.push(act);
                    }

                    js = serde_json::Value::Array(vec_v)
                }
                _ => {
                    //decode struct
                    let size = self.len();
                    if size > 1 {
                        return Result::Err("[rbatis] rows.affected_rows > 1,but decode one result!".to_string());
                    }
                    for i in 0..size {
                        let item = self.get(i);
                        js = decode_row(&item.unwrap());
                    }
                }
            }
        }
        let decode_result = serde_json::from_value(js);
        if decode_result.is_ok() {
            return Result::Ok(decode_result.unwrap());
        } else {
            let e = decode_result.err().unwrap().to_string();
            return Result::Err(e);
        }
    }
}

fn decode_row(row: &Row) -> Value {
    let cs = row.columns();
    let mut m = serde_json::map::Map::new();
    let mut index = 0;
    for c in cs.as_ref() {
        let column_name = c.name();
        let c_type = cs.get(index).unwrap().type_().name();
        let mut v = serde_json::Value::Null;
        // println!("c_type:{}",c_type);
        if c_type == "varchar" {
            let field: Option<String> = row.get(index);
            if field.is_some() {
                v = serde_json::Value::String(field.unwrap());
            }
        } else if c_type == "int2" {
            let field: Option<i16> = row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "int4" {
            let field: Option<i32> = row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "int8" {
            let field: Option<i64> = row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "float4" {
            let field: Option<f32> = row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "float8" {
            let field: Option<f64> = row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }
        m.insert(column_name.to_string(), v);
        index = index + 1;
    }
    return serde_json::Value::Object(m);
}