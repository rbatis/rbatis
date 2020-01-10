use serde_json::json;
use serde_json::Value;

use crate::convert::sql_value_convert::{SqlColumnConvert, SqlValueConvert};
use crate::core::rbatis::Rbatis;
use serde::de::DeserializeOwned;
use std::fs;

impl Rbatis {

    pub fn insert<T>(&mut self, mapper_name: &str, arg: &mut Value) -> Result<T, String> where T: DeserializeOwned {
        let sql = self.create_sql_insert(mapper_name, arg)?;
        let mut arg_array=vec![];
        return self.eval_sql_raw((mapper_name.to_string()+".insert").as_str(),sql.as_str(), true,&mut arg_array);
    }

    pub fn create_sql_insert(&mut self, mapper_name: &str, arg: &mut Value) -> Result<String, String> {
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }
        let result_map_node = self.get_result_map_node(mapper_name)?;
        let mut sql = "insert into #{table} (#{fields}) VALUES #{values}".to_string();
        if result_map_node.table.is_none() {
            return Result::Err("[rbatis]  can not find table defin in <result_map>!".to_string());
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());
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
                let value_item_sql = do_create_values_sql(x);
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


fn do_create_obj_sql(mut sql: String, arg: &Value) -> String {
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

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let mut arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let sql = rbatis.create_sql_insert("Example_ActivityMapper.xml", &mut arg).unwrap();
    println!("{}", sql);
}

#[test]
fn test_insert_templete_array() {

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let mut arg = serde_json::from_str(r#"[{"a":"1","delete_flag":1},{"a":"1","delete_flag":1}]"#).unwrap();
    let sql = rbatis.create_sql_insert("Example_ActivityMapper.xml", &mut arg).unwrap();
    println!("{}", sql);
}