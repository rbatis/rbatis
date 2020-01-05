use serde_json::Value;

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::convert::sql_value_convert;
use crate::convert::sql_value_convert::SqlValueConvert;
use crate::core::rbatis::Rbatis;
use std::fs;
use std::borrow::BorrowMut;

impl Rbatis {
    pub fn select(&mut self, mapper_name: &str, id: &str, arg: &mut Value) -> Result<String, String> {
        //TODO select by page
        let result_map_node=self.get_result_map_node(mapper_name,id)?;
        match arg {
            serde_json::Value::Null => {
                return Result::Err("[rbatis] arg is null value".to_string());
            }
            serde_json::Value::String(_) | serde_json::Value::Number(_) => {
                let mut where_str= "id = ".to_string()+ arg.to_sql_value_custom("").as_str();
                return Result::Ok(self.do_select_by_templete(arg,&result_map_node,where_str.as_str())?);
            }
            serde_json::Value::Array(_) => {
                let mut where_str= "id in ".to_string()+ arg.to_sql_value_custom("").as_str();
                return Result::Ok(self.do_select_by_templete(arg,&result_map_node,where_str.as_str())?);
            }
            serde_json::Value::Object(map) => {
                let mut where_str= arg.to_sql_value_custom("");
                return Result::Ok(self.do_select_by_templete(arg,&result_map_node,where_str.as_str())?);
            }

            _ => {
                return Result::Err("[rbatis] not support arg type value in select(): ".to_string() + arg.to_sql_value().as_str());
            }
        }
        return Result::Err("[rbatis] eval select crud fail".to_string());
    }

    fn do_select_by_templete(&mut self, env: &mut Value, result_map_node: &ResultMapNode, where_str: &str) -> Result<String, String> {
        let mut sql = "select * from #{table} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err("[rbatis]  can not find table defin in <result_map>!".to_string());
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());

        //replace where
        let mut where_string = where_str.to_string();
        where_string.trim();
        //delete node
        if result_map_node.delete_node.is_some() {
            if !where_string.is_empty() {
                where_string += sql_value_convert::AND;
            }
            where_string = where_string + result_map_node.delete_node.as_ref().unwrap().column.as_str() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str();
        }
        //replace where
        sql = sql.replace("#{where}", where_string.as_str());

        return Result::Ok(sql);
    }
}

#[test]
fn test_select_by_id(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql=rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!("1").borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_select_by_ids(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql=rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(vec![1,2,3]).borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_select_by_map(){
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql=rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!({
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"]
    }).borrow_mut());
    println!("{}",sql.unwrap());
}