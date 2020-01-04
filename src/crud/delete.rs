use std::borrow::BorrowMut;
use std::fs;

use serde_json::Value;

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::core::rbatis::Rbatis;
use crate::engine::node::NodeType;

impl Rbatis{
    pub fn delete(&mut self, mapper_name: &str, id: &str, arg: &mut Value)-> Result<String, String>{
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }

        let result_map_node=self.get_result_map_node(mapper_name,id)?;
        println!("{:?}",result_map_node);

        match arg {
            serde_json::Value::String(str)=>{
                return self.do_delete_by_id(mapper_name,id,arg,&result_map_node);
            }
            _ => {
                return Result::Err("[rbatis] not support arg type".to_string());
            }
        };
        //TODO delete by id

        //TODO delete by map
        //TODO delete by ids
    }

    fn do_delete_by_id(&mut self, mapper_name: &str, id: &str, env: &mut Value,result_map_node:&ResultMapNode)-> Result<String, String>{
        let mut sql = "DELETE FROM #{table} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err("[rbatis]  can not find table defin in <result_map>!".to_string());
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());
        //replace where
        let mut where_str = "id = ".to_string() + env.as_str().unwrap();
        let id = env.as_str().unwrap();
        if result_map_node.delete_node.is_some() {
            let column = result_map_node.delete_node.as_ref().unwrap();
            where_str = where_str + " and " + column.column.as_str() + " = " + env.as_str().unwrap();
        }
        sql = sql.replace("#{where}", where_str.as_ref());
        return Result::Ok(sql);
    }
}

#[test]
fn test_delete_by_id() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql=rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"1234123123"#).borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_delete_by_ids() {
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"1234123123"#).borrow_mut());
}

#[test]
fn test_delete_by_map() {
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"1234123123"#).borrow_mut());
}