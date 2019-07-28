use std::sync::Arc;
use mysql::{Column, Value, Row, QueryResult};
use std::result;
use serde::de;
use std::any::Any;



pub fn decode<T>(rows:QueryResult, r: &mut T) -> Option<serde_json::Error>
    where
        T: de::DeserializeOwned {
    let mut js = "[".to_owned();
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
    let decodeR= serde_json::from_str(js.as_str());
    if decodeR.is_ok(){
        *r=decodeR.unwrap();
    }else{
        return decodeR.err();
    }
    return None
}

pub fn decodeRow(row: &Row) -> String {
    let cs = row.columns();
    let csLen=cs.len();

    let mut json_obj_str = String::new();
    for c in cs.as_ref() {
        let columnName = c.name_str();
        let k = columnName.as_ref();
        let f: Value = row.get(k).unwrap();
        json_obj_str = json_obj_str + "\"" + columnName.as_ref() + "\"";
        let mut sql = f.as_sql(true);
        if sql.as_str() == "NULL" {
            sql = "null".to_string();
        } else {
            if sql == "''" {
                sql = "\"\"".to_owned();
            } else {
                let sqlLen = sql.len();
                let first = sql.find("'").unwrap_or_default();
                let last = sql.rfind("'").unwrap_or_default();
                if first == 0 && last == (sqlLen - 1) && first != last {
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