use serde_json::Value;
use serde_json::json;
use crate::core::rbatis::Rbatis;
use crate::convert::sql_value_convert::SqlValueConvert;


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
            //todo replace to xml field
            if !arr.get(0).unwrap().is_object() {
                return Result::Err("[rbatis] unsupport arg type,only support object json and object array!".to_string());
            }
            let fields = do_create_field_sql(arr.get(0).unwrap().clone());
            sql = sql.replace("#{fields}", fields.as_str());

            for x in arr {
                if !x.is_object() {
                    return Result::Err("[rbatis] unsupport arg type,only support object json and object array!".to_string());
                }
                let mut value_item_sql = do_create_values_sql(x);
                value_item_sql = "(".to_string() + value_item_sql.as_str() + "),";
                values.push_str(value_item_sql.as_str());
            }
            values.pop();
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
    let mut values = "".to_string();
    let len = obj_map.len();
    let mut i = 0;
    for (_, v) in obj_map {
        let vstr=v.to_sql();
        if i < (len - 1) {
            values = values + vstr.as_str() + ",";
        } else {
            values = values + vstr.as_str();
        }
        i = i + 1;
    }
    return values;
}

//todo replace to xml field
fn do_create_field_sql(arg: Value) -> String {
    let obj = arg.as_object();
    let obj_map = obj.unwrap();
    let mut fields = "".to_string();
    let len = obj_map.len();
    let mut i = 0;
    for (x, _) in obj_map {
        if i < (len - 1) {
            fields = fields + x.as_str() + ",";
        } else {
            fields = fields + x.as_str();
        }
        i = i + 1;
    }
    return fields;
}

fn do_create_obj_sql(mut sql: String, arg: Value) -> String {
    let obj = arg.as_object();
    let obj_map = obj.unwrap();
    //todo replace to xml field
    let mut fields = "".to_string();
    let mut values = "".to_string();
    let len = obj_map.len();
    let mut i = 0;
    for (x, v) in obj_map {
        let vstr = v.to_sql();
        if i < (len - 1) {
            fields = fields + x.as_str() + ",";
            values = values + vstr.as_str() + ",";
        } else {
            fields = fields + x.as_str();
            values = values + vstr.as_str();
        }
        i = i + 1;
    }
    sql = sql.replace("#{fields}", fields.as_str());
    sql = sql.replace("#{values}", ("(".to_string() + values.as_str() + ")").as_str());
    return sql;
}


#[test]
fn test_insert_templete_obj() {
    let rbatis=Rbatis::new();
    let arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let sql = rbatis.insert("activity", arg).unwrap();
    println!("{}", sql);
}

#[test]
fn test_insert_templete_array() {
    let rbatis=Rbatis::new();
    let arg = serde_json::from_str(r#"[{"a":"1","delete_flag":1},{"a":"1","delete_flag":1}]"#).unwrap();
    let sql = rbatis.insert("activity", arg).unwrap();
    println!("{}", sql);
}