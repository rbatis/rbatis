use serde_json::json;
use serde_json::Value;

use crate::convert::sql_value_convert::{SqlColumnConvert, SqlValueConvert};
use crate::core::rbatis::Rbatis;

impl Rbatis {
    /// insert an object or  array
    /// example:
    /// rbatis.insert("activity", json!(arg))
    /// rbatis.insert("activity", json!(vec![arg1,arg2]))
    pub fn insert(&self, table: &str, arg: Value) -> Result<String, String> {
        let mut sql = "insert into #{table} (#{fields}) VALUES #{values}".to_string();
        sql = sql.replace("#{table}", table);

        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }
        if arg.is_object() {
            //insert object
            return Result::Ok(do_create_obj_sql(sql, arg));
        } else if arg.is_array() {
            //insert array
            let mut values = "".to_string();
            let arr = arg.as_array().unwrap();
            if arr.len() == 0 {
                return Result::Err("[rbatis] arg array len = 0!".to_string());
            }
            if !arr.get(0).unwrap().is_object() {
                return Result::Err("[rbatis] unsupport arg type,only support object json and object array!".to_string());
            }
            let fields = arr.get(0).unwrap().clone().to_sql_column();
            sql = sql.replace("#{fields}", fields.as_str());

            let mut append = false;
            for x in arr {
                if !x.is_object() {
                    return Result::Err("[rbatis] unsupport arg type,only support object json and object array!".to_string());
                }
                let mut value_item_sql = do_create_values_sql(x);
                values = values + value_item_sql.as_str() + ",";
                append = true;
            }
            if append {
                values.pop();
            }
            sql = sql.replace("#{values}", values.as_str());
            return Result::Ok(sql);
        } else {
            return Result::Err("[rbatis] unsupport arg type,only support object json and object array!".to_string());
        }
    }
}


fn do_create_values_sql(arg: &Value) -> String {
    let obj = arg.as_object();
    let obj_map = obj.unwrap();
    let mut values = vec![];
    for (_, v) in obj_map {
        values.push(v.clone());
    }
    return serde_json::Value::Array(values).to_sql_value_skip("");
}


fn do_create_obj_sql(mut sql: String, arg: Value) -> String {
    let obj = arg.as_object();
    let obj_map = obj.unwrap();
    let mut values = vec![];
    for (x, item) in obj_map {
        values.push(item.clone());
    }
    let vals = serde_json::Value::Array(values);
    sql = sql.replace("#{fields}", arg.to_sql_column().as_str());
    sql = sql.replace("#{values}", vals.to_sql_value_skip("").as_str());
    return sql;
}


#[test]
fn test_insert_templete_obj() {
    let rbatis = Rbatis::new();
    let arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let sql = rbatis.insert("activity", arg).unwrap();
    println!("{}", sql);
}

#[test]
fn test_insert_templete_array() {
    let rbatis = Rbatis::new();
    let arg = serde_json::from_str(r#"[{"a":"1","delete_flag":1},{"a":"1","delete_flag":1}]"#).unwrap();
    let sql = rbatis.insert("activity", arg).unwrap();
    println!("{}", sql);
}