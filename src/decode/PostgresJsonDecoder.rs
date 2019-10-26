
use crate::decode::Decoder::{Decoder, isJsonArrayType};
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
        if isJsonArrayType(T::decode_name()) {
            //is array json
            let mut vec_v = vec![];
            for i in 0..self.len(){
                let mut item=self.get_mut(i);
                let act= decodeRow(&item.unwrap());
                vec_v.push(act);
            }
            js = serde_json::Value::Array(vec_v);
        }else{
            let mut result: Result<T, String> = Result::Err("[Rbatis] rows.affected_rows > 1,but decode one result!".to_string());
            //not array json
            let size=self.len();
            if size > 1 {
                return result;
            }
            for i in 0..size {
                let item = self.get(i);
                js = decodeRow(&item.unwrap());
            }
        }
        let decodeR = serde_json::from_value(js);
        if decodeR.is_ok() {
            return Result::Ok(decodeR.unwrap());
        } else {
            let e = decodeR.err().unwrap().to_string();
            return Result::Err(e);
        }
    }

}

fn decodeRow(row: &Row) -> Value {
    let cs = row.columns();
    let mut m = serde_json::map::Map::new();
    let mut index=0;
    for c in cs.as_ref() {
        let columnName = c.name();
        let c_type= cs.get(index).unwrap().type_().name();
        let mut v=serde_json::Value::Null;
        // println!("c_type:{}",c_type);
        if  c_type == "varchar" {
            let mut field:String=row.get(index);
            if !field.eq("null") {
                v = serde_json::Value::String(field);
            }
        }else if c_type == "int2"{
            let mut field:Option<i16>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "int4"{
            let mut field:Option<i32>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "int8"{
            let mut field:Option<i64>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }else if c_type == "float4"{
            let mut field:Option<f32>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        } else if c_type == "float8"{
            let mut field:Option<f64>=row.get(index);
            if field.is_some() {
                v = json!(field.unwrap());
            }
        }
        m.insert(columnName.to_string(), v);
        index=index+1;
    }
    return serde_json::Value::Object(m);
}