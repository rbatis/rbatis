use std::borrow::{BorrowMut, Borrow};
use std::fs;

use serde_json::{Value, Map};

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::core::rbatis::Rbatis;
use crate::engine::node::NodeType;
use serde_json::value::Value::Number;
use crate::convert::sql_value_convert::SqlValueConvert;
use std::any::Any;
use serde::de::DeserializeOwned;

impl Rbatis{

    pub fn delete<T>(&mut self, mapper_name: &str, arg: &mut Value) -> Result<T, String> where T: DeserializeOwned {
        let sql = self.create_sql_delete(mapper_name, arg)?;
        let db = self.get_conf("delete");
        return self.eval_sql_raw(sql.as_str(), true,db.as_str());
    }


    pub fn create_sql_delete(&mut self, mapper_name: &str, arg: &mut Value) -> Result<String, String>{
        let result_map_node=self.get_result_map_node(mapper_name)?;
        match arg {
            serde_json::Value::String(_) | serde_json::Value::Number(_)=>{
                //delete by id
                //replace where
                let where_str = "id = ".to_string() + arg.to_sql_value_def().as_str();
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Array(_)=>{
                //delete by ids
                let where_str="id in ".to_string()+arg.to_sql_value_def().as_str();
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Object(map)=>{
                let  c=map.clone();
                let where_str=arg.to_sql_value_def();
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Null=>{
                return Result::Err("[rbatis] delete arg type can not be null!".to_string());
            }
            _ => {
                return Result::Err("[rbatis] not support arg type value in delete(): ".to_string()+arg.to_sql_value_def().as_str());
            }
        };
    }


    ///基本删除语句模板
    fn do_delete_by(&mut self, env: &mut Value,result_map_node:&ResultMapNode,where_str:&str)-> Result<String, String>{
        let mut sql = "DELETE FROM #{table} #{set} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err("[rbatis]  can not find table defin in <result_map>!".to_string());
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());
        //delete node
        if result_map_node.delete_node.is_some(){
            sql=sql.replace("DELETE FROM","UPDATE");
            sql=sql.replace("#{set}",("SET ".to_string() +result_map_node.delete_node.as_ref().unwrap().column.as_str() + " = "+ result_map_node.delete_node.as_ref().unwrap().logic_deleted.as_str()).as_str());
        }else{
            sql=sql.replace("#{set}","");
        }
        //replace where
        sql = sql.replace("#{where}", where_str);
        return Result::Ok(sql);
    }
}


#[test]
fn test_delete_by_id() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql=rbatis.create_sql_delete("Example_ActivityMapper.xml", serde_json::json!("1").borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_delete_by_ids() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql =rbatis.create_sql_delete("Example_ActivityMapper.xml", serde_json::json!(vec![1,2,3]).borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_delete_by_map() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql =rbatis.create_sql_delete("Example_ActivityMapper.xml", serde_json::json!({
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"]
    }).borrow_mut());
    println!("{}",sql.unwrap());
}