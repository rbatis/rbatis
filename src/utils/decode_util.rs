use std::sync::Arc;
use mysql::{Column, Value, Row, QueryResult};
use std::result;
use serde::de;
use std::any::Any;

use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;

pub type Error = String;


/**
* the json decode util
* by  zhuxiujia@qq.com
**/
pub fn decode<T>(rows: QueryResult) -> Result<T, Error>
    where
        T: de::DeserializeOwned + RbatisMacro {
    let mut js = "".to_owned();
    if T::decode_name() == "Vec" || T::decode_name() == "Array" || T::decode_name() == "Slice" || T::decode_name() == "LinkedList" {
        //is array json
        js = "[".to_owned();
        let mut push_spar = false;
        rows.for_each(|item| {
            let row = item.unwrap();
            let act = decodeRow(&row);
            js.push_str(act.as_str());
            js.push_str(",");
            push_spar = true;
        });
        if push_spar {
            js.pop();
        }
        js = js + "]";
    } else {
        //not array json
        let mut index = 0;
        rows.for_each(|item| {
            if index > 1 {
                return;
            }
            let row = item.unwrap();
            let act = decodeRow(&row);
            js.push_str(act.as_str());
            index = index + 1;
        });
        if index > 1 {
            return Result::Err("rows.affected_rows > 1,but decode one result!".to_string());
        }
    }
    let decodeR = serde_json::from_str(js.as_str());
    if decodeR.is_ok() {
        return Result::Ok(decodeR.unwrap());
    } else {
        let e = decodeR.err().unwrap().to_string();
        return Result::Err(e);
    }
}

pub fn decodeRow(row: &Row) -> String {
    let cs = row.columns();
    let csLen = cs.len();

    let mut json_obj_str = String::new();
    for c in cs.as_ref() {
        let columnName = c.name_str();
        let k = columnName.as_ref();
        let f: Value = row.get(k).unwrap();
        json_obj_str = json_obj_str + "\"" + columnName.as_ref() + "\"";
        let mut sql = f.as_sql(true);
        let sqlLen = sql.len();
        if sql.as_str() == "NULL" {
            sql = "null".to_string();
        } else {
            if sql == "''" {
                sql = "\"\"".to_owned();
            } else if sql.starts_with("'") {
                if sql.ends_with("'") && sqlLen > 1 {
                    let slice = &sql[1..(sqlLen - 1)];
                    sql = "\"".to_owned() + slice + "\"";
                }
            }
        }
        json_obj_str = json_obj_str + ":" + sql.as_str() + ",";
    }
    json_obj_str.pop();
    json_obj_str = "{".to_owned() + json_obj_str.as_str() + "}";
    return json_obj_str;
}