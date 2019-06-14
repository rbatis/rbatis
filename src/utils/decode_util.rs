use std::sync::Arc;
use mysql::{Column, Value, Row};
use std::result;
use serde::de;


pub fn decode<T>(row: &Row) -> result::Result<T, serde_json::Error>
    where
        T: de::DeserializeOwned {
    let mut json_obj_str = String::new();
    let cs = row.columns();
    for c in cs.as_ref() {
        let columnName = c.name_str();
        println!("{:?}", columnName);
        let k = columnName.as_ref();
        let f: Value = row.get(k).unwrap();
        println!("{:?}", f);

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
    println!("query sql result ==> {}", json_obj_str);

    return serde_json::from_str(json_obj_str.as_str());
}