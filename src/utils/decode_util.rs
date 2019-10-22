use std::sync::Arc;
use mysql::{Column, Value, Row, QueryResult};
use std::result;
use serde::de;
use std::any::Any;

use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use std::collections::HashMap;
use serde_json::Number;
use std::str::FromStr;

pub type Error = String;


#[derive(Clone, PartialEq)]
pub struct RQueryResult{
    rows:Vec<Row>,
}

impl RQueryResult{
    pub fn from_query_result(arg:QueryResult)->Self{
        let mut rq=RQueryResult{
            rows: vec![],
        };
        arg.for_each(|item|{
            rq.rows.push(item.unwrap());
        });
        return rq;
    }
}


/**
* the json decode util
* by  zhuxiujia@qq.com
**/
pub fn decode<T>(rows: RQueryResult) -> Result<T, Error>
    where
        T: de::DeserializeOwned + RbatisMacro {
    let mut js = serde_json::Value::Null;
    if T::decode_name() == "Vec" || T::decode_name() == "Array" || T::decode_name() == "Slice" || T::decode_name() == "LinkedList" {
        //is array json
        let mut vec_v=vec![];
        for item in rows.rows{
            let act = decodeRow(&item);
            vec_v.push(act);
        }
        js=serde_json::Value::Array(vec_v)
    } else {
        //not array json
        let mut index = 0;
        for item in rows.rows{
            if index > 1 {
                return Result::Err("rows.affected_rows > 1,but decode one result!".to_string());
            }
            js = decodeRow(&item);
            index = index + 1;
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

pub fn decodeRow(row: &Row) -> serde_json::Value {
    let cs = row.columns();
    let mut m=serde_json::map::Map::new();
    for c in cs.as_ref() {
        let columnName = c.name_str();
        let k = columnName.as_ref();
        let f: Value = row.get(k).unwrap();

        let mut sql = f.as_sql(true);
        let sqlLen = sql.len();
        let item:serde_json::Value;
        if sql.as_str() == "NULL" {
            item=serde_json::Value::Null;
        } else {
            if sql == "''" {
                sql = "\"\"".to_owned();
                item=serde_json::Value::String(sql);
            } else if sql.starts_with("'") {
                let slice = &sql[1..(sqlLen - 1)];
                sql = "\"".to_owned() + slice + "\"";
                item=serde_json::Value::String(sql);
            }else{
                let n=Number::from_str(sql.as_str());
                if n.is_ok(){
                    item=serde_json::Value::Number(n.unwrap());
                }else{
                    item=serde_json::Value::Null;
                }
            }
        }
       m.insert(columnName.to_string(),item);
    }
    return serde_json::Value::Object(m);
}