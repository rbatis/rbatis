
use crate::decode::decoder::{Decoder, is_json_array_type};
use serde::de;
use rbatis_macro::RbatisMacro;
use std::borrow::BorrowMut;
use serde_json::Value;
use postgres::Row;
use std::str::FromStr;
use serde_json::json;
use serde_json::value::Value::Number;


//PG 解析器
impl Decoder for Vec<Row>{
    fn decode<T>(&mut self) -> Result<T, String> where
        T: de::DeserializeOwned + RbatisMacro {
        //unimplemented!();
        let mut js = serde_json::Value::Null;
        if is_json_array_type(T::decode_name()) {
            //is array json
            let mut vec_v = vec![];
            for i in 0..self.len(){
                let mut item=self.get_mut(i);
                let act= decode_row(&item.unwrap());
                vec_v.push(act);
            }
            js = serde_json::Value::Array(vec_v);
        }else{
            let mut result: Result<T, String> = Result::Err("[rbatis] rows.affected_rows > 1,but decode one result!".to_string());
            //not array json
            let size=self.len();
            if size > 1 {
                return result;
            }
            for i in 0..size {
                let item = self.get(i);
                js = decode_row(&item.unwrap());
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
    let mut index=0;
    for c in cs.as_ref() {
        let column_name = c.name();
        let c_type= cs.get(index).unwrap().type_().name();
        let mut v=serde_json::Value::Null;
        // println!("c_type:{}",c_type);
        if  c_type == "varchar" {
            let field:Option<String>=row.get(index);
            if field.is_some() {
                v = serde_json::Value::String(field.unwrap());
            }
        }else if c_type == "int2"{
            let field:Option<i16>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "int4"{
            let field:Option<i32>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "int8"{
            let field:Option<i64>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "float4"{
            let field:Option<f32>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "float8"{
            let field:Option<f64>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }
        m.insert(column_name.to_string(), v);
        index=index+1;
    }
    return serde_json::Value::Object(m);
}