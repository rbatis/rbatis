use serde_json::json;
use serde_json::Value;

use crate::convert::sql_value_convert::{SqlColumnConvert, SqlValueConvert, SqlQuestionConvert, SkipType, AND};
use crate::core::rbatis::Rbatis;
use serde::de::DeserializeOwned;
use std::fs;
use crate::utils::rdbc_util::rdbc_vec_to_string;
use crate::utils::string_util::count_string_num;

impl Rbatis {
    pub fn insert<T>(&mut self, mapper_name: &str, arg: &mut Value) -> Result<T, String> where T: DeserializeOwned {
        let mut arg_array = vec![];
        let sql = self.create_sql_insert(mapper_name, arg, &mut arg_array)?;
        return self.eval_raw((mapper_name.to_string() + ".insert").as_str(), sql.as_str(), false, &mut arg_array);
    }

    pub fn create_sql_insert(&mut self, mapper_name: &str, arg: &mut Value, arg_array: &mut Vec<Value>) -> Result<String, String> {
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
            return Result::Ok(do_create_obj_sql(sql, arg, arg_array));
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
                let obj=x.as_object().unwrap();
                let mut obj_vec=vec![];
                for (_,item) in obj {
                    obj_vec.push(item.clone());
                }
                let value_item_sql = serde_json::Value::Array(obj_vec).to_sql_question(SkipType::None,",",",",arg_array);
                values = values +value_item_sql.as_str() + ",";
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


fn do_create_obj_sql(mut sql: String, arg: &Value, arg_array: &mut Vec<Value>) -> String {
    let values = arg.to_sql_question(SkipType::None,AND,",",arg_array);
    sql = sql.replace("#{fields}", arg.to_sql_column().as_str());
    sql = sql.replace("#{values}", ("(".to_string() + values.as_str() + ")").as_str());
    return sql;
}


#[test]
fn test_insert_templete_obj() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let mut arg = serde_json::from_str(r#"{  "a":"1","delete_flag":1}"#).unwrap();
    let mut arg_array = vec![];
    let sql = rbatis.create_sql_insert("Example_ActivityMapper.xml", &mut arg, &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));
    assert_eq!(arg_array.len(),count_string_num(&sql,'?'));
}

#[test]
fn test_insert_templete_array() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    let mut arg_array = vec![];
    let mut arg = serde_json::from_str(r#"[{"a":"1","delete_flag":1},{"a":"1","delete_flag":1}]"#).unwrap();
    let sql = rbatis.create_sql_insert("Example_ActivityMapper.xml", &mut arg, &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));
    assert_eq!(arg_array.len(),count_string_num(&sql,'?'));
}