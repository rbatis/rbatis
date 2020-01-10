use std::borrow::BorrowMut;
use std::fs;

use serde_json::{Map, Number, Value};

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::convert::sql_value_convert::{AND, SKIP_TYPE_ARRAY, SKIP_TYPE_NULL, SKIP_TYPE_OBJECT, SqlColumnConvert, SqlValueConvert};
use crate::convert::sql_value_convert;
use crate::core::rbatis::Rbatis;
use serde::de::DeserializeOwned;

pub const SKIP_SETS: &'static str = "null,object,array";

impl Rbatis {

    pub fn update<T>(&mut self, mapper_name: &str,  arg: &mut Value) -> Result<T, String> where T: DeserializeOwned {
        let sql = self.create_sql_update(mapper_name, arg)?;
        let mut arg_array=vec![];
        return self.eval_sql_raw(mapper_name,sql.as_str(), true,&mut arg_array);
    }



    pub fn create_sql_update(&mut self, mapper_name: &str, arg: &mut Value) -> Result<String, String> {
        let result_map_node = self.get_result_map_node(mapper_name)?;
        match arg {
            serde_json::Value::Array(arr) => {
                let mut sqls = "".to_string();
                //delete by ids
                for x in arr {
                    match x {
                        serde_json::Value::Object(_) => {
                            let temp_sql = self.create_sql_update(mapper_name, x)?;
                            sqls = sqls + temp_sql.as_str() + "; \n";
                        }
                        _ => {
                            return Result::Err("[rbatis] update only support object or array,not support arg type value in update(): ".to_string() + arg.to_sql_value_def().as_str());
                        }
                    }
                }
                return Result::Ok(sqls);
            }
            serde_json::Value::Object(map) => {
                let mut where_str = "".to_string();
                let c = map.clone();
                if result_map_node.id_node.is_some() {
                    let id_value = c.get(&result_map_node.id_node.as_ref().unwrap().property);
                    if id_value.is_none() {
                        return Result::Err("[rbatis] arg id field:".to_string() + result_map_node.id_node.as_ref().unwrap().property.as_str() + " can not be null in update()!");
                    }
                    where_str = where_str + "id = " + id_value.unwrap().to_sql_value_skip("").as_str();
                } else {
                    where_str = where_str + arg.to_sql_value_skip("").as_str();
                }
                let mut sets_map = Map::new();
                for (k, v) in c {
                    if result_map_node.id_node.is_some() && result_map_node.id_node.as_ref().unwrap().column.eq(&k) {
                        continue;
                    }
                    if result_map_node.delete_node.is_some() && result_map_node.delete_node.as_ref().unwrap().column.eq(&k) {
                        continue;
                    }
                    if result_map_node.version_node.is_some() && result_map_node.version_node.as_ref().unwrap().column.eq(&k) {
                        continue;
                    }
                    if v.is_null() {
                        continue;
                    }
                    sets_map.insert(k, v);
                }
                let sets_object = Value::Object(sets_map);
                return self.do_update_by(arg, &result_map_node, sets_object.to_sql_value_custom(SKIP_SETS, ",", ",").as_str(), where_str.as_str());
            }
            serde_json::Value::Null => {
                return Result::Err("[rbatis] delete arg type can not be null!".to_string());
            }
            _ => {
                return Result::Err("[rbatis] update only support object or array,not support arg type value in update(): ".to_string() + arg.to_sql_value_def().as_str());
            }
        };
    }


    ///基本删除语句模板
    fn do_update_by(&mut self, env: &mut Value, result_map_node: &ResultMapNode, sets: &str, where_str: &str) -> Result<String, String> {
        let mut sql = "UPDATE #{table} #{set} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err("[rbatis]  can not find table defin in <result_map>!".to_string());
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());

        //build set
        let mut sets = sets.to_string();
        //version node
        if result_map_node.version_node.is_some() {
            let version_value_opt = get_version_value(env, &result_map_node.version_node.as_ref().unwrap().property);
            if version_value_opt.is_some() {
                let v = version_value_opt.unwrap();
                match v {
                    serde_json::Value::Number(n) => {
                        if !sets.is_empty() {
                            sets = sets + ","
                        }
                        if n.is_f64() {
                            return Result::Err("[rbatis] not support version value type,version field must be a i64 or u64 number!!".to_string());
                        } else if n.is_u64() {
                            sets = sets + " " + result_map_node.version_node.as_ref().unwrap().column.as_str() + " = " + (n.as_u64().unwrap() + 1 as u64).to_string().as_str();
                        } else {
                            //i64
                            sets = sets + " " + result_map_node.version_node.as_ref().unwrap().column.as_str() + " = " + (n.as_i64().unwrap() + 1 as i64).to_string().as_str();
                        }
                    }
                    serde_json::Value::Null => {}
                    _ => {
                        return Result::Err("[rbatis] not support version value type,version field must be a i64 or u64 number!".to_string());
                    }
                }
            }
        }
        sql = sql.replace("#{set}", ("SET ".to_string() + sets.as_str()).as_str());
        //replace where
        let mut where_string = where_str.to_string();
        //version node
        if result_map_node.version_node.is_some() && !where_string.is_empty() {
            let version_value_opt = get_version_value(env, &result_map_node.version_node.as_ref().unwrap().property);
            if version_value_opt.is_some() {
                let version = version_value_opt.unwrap();
                where_string = where_string + AND + result_map_node.version_node.as_ref().unwrap().column.as_str() + " = " + version.to_sql_value_def().as_str();
            }
        }
        //delete node
        if result_map_node.delete_node.is_some() && !where_string.is_empty() {
            where_string = where_string + AND + result_map_node.delete_node.as_ref().unwrap().column.as_str() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str();
        }
        sql = sql.replace("#{where}", where_string.as_str());
        return Result::Ok(sql);
    }
}

fn get_version_value(env: &Value, property: &String) -> Option<Value> {
    match env {
        Value::Object(m) => {
            let c = m.get(property);
            if c.is_some() {
                return Some(c.unwrap().clone());
            }
            return None;
        }
        _ => {
            return None;
        }
    }
}


#[test]
fn test_update_by_id() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql = rbatis.create_sql_update("Example_ActivityMapper.xml",  serde_json::json!({
     "id":"1",
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"],
     "version":2,
    }).borrow_mut());
    println!("{}", sql.unwrap());
}

#[test]
fn test_update_by_ids() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let mut json_arr = serde_json::from_str(r#"[
    {
     "id":"1",
     "arg": 2,
     "delete_flag":1,
     "number_arr":[1,2,3],
     "string_arr":["1","2","3"],
     "version":2
    },
    {
     "id":"1",
     "arg": 2,
     "delete_flag":1,
     "number_arr":[1,2,3],
     "string_arr":["1","2","3"],
     "version":2
    }
    ]"#).unwrap();
    let sql = rbatis.create_sql_update("Example_ActivityMapper.xml",  &mut json_arr);
    println!("{}", sql.unwrap());
}