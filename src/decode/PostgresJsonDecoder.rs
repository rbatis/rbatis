
use crate::decode::Decoder::{Decoder, isJsonArrayType};
use postgres::rows::{Rows, Row};
use serde::de;
use rbatis_macro::RbatisMacro;
use std::borrow::BorrowMut;
use serde_json::Value;


//PG 解析器
impl Decoder for Rows{
    fn decode<T>(&mut self) -> Result<T, String> where
        T: de::DeserializeOwned + RbatisMacro {
        //unimplemented!();
        let mut js = serde_json::Value::Null;
        if isJsonArrayType(T::decode_name()) {
            //is array json
            let mut vec_v = vec![];
            for i in 0..self.len(){
                let item=self.get(i);
                let act= decodeRow(&item);
                vec_v.push(act);
            }
            js = serde_json::Value::Array(vec_v);
        }else{
            let mut result: Result<T, String> = Result::Err("[Rbatis] rows.affected_rows > 1,but decode one result!".to_string());
            //not array json
            let mut index = 0;
            for i in 0..self.len() {
                let item = self.get(i);
                if index > 1 {
                    continue;
                }
                js = decodeRow(&item);
                index = index + 1;
            }
            if index > 0 {
                return result;
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

        if index==0{
            let id:Value= row.get(index);
            println!("id:{}",id);
        }



        let fieldOpt:Option<postgres::Result<Value>>=row.get_opt(index);
        let mut field=fieldOpt.unwrap_or(Result::Ok(Value::Null));
        m.insert(columnName.to_string(), field.unwrap());
        index=index+1;
    }
    return serde_json::Value::Object(m);
}