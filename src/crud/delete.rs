use std::borrow::BorrowMut;
use std::fs;

use serde_json::Value;

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::core::rbatis::Rbatis;
use crate::engine::node::NodeType;
use serde_json::value::Value::Number;

impl Rbatis{
    pub fn delete(&mut self, mapper_name: &str, id: &str, arg: &mut Value)-> Result<String, String>{
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }
        let result_map_node=self.get_result_map_node(mapper_name,id)?;
        match arg {
            serde_json::Value::String(str)=>{
                //delete by id
                return self.do_delete_by_id(arg,&result_map_node);
            }
            serde_json::Value::Array(arr)=>{
                //delete by ids
                let mut string_arr=vec![];
                for item in arr {
                    match item {
                        serde_json::Value::String(arr_str)=>{
                            string_arr.push(item.clone());
                        },
                        serde_json::Value::Number(number)=>{
                            if number.is_f64(){
                                string_arr.push(item.clone());
                            }else if number.is_i64(){
                                string_arr.push(item.clone());
                            }else if number.is_u64(){
                                string_arr.push(item.clone());
                            } else{
                                return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                            }
                        }
                        _ => {
                            return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                        }
                    }
                }
                return self.do_delete_by_ids(arg,&result_map_node,string_arr);
            }
            serde_json::Value::Object(map)=>{
                //TODO delete by ids
                return Result::Err("".to_string());
            }
            _ => {
                return Result::Err("[rbatis] not support arg type".to_string());
            }
        };
    }

    fn do_delete_by_id(&mut self, env: &mut Value,result_map_node:&ResultMapNode)-> Result<String, String>{
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
        let mut where_str = "id = ".to_string() + env.as_str().unwrap();
        let id = env.as_str().unwrap();
        sql = sql.replace("#{where}", where_str.as_ref());
        return Result::Ok(sql);
    }

    fn do_delete_by_ids(&mut self, env: &mut Value,result_map_node:&ResultMapNode, arr:Vec<Value>)-> Result<String, String>{
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
        let mut where_str = "id in (".to_string();
        for x in arr {
            match x{
                serde_json::Value::String(x)=>{
                    where_str=where_str+x.as_str()+",";
                }
                serde_json::Value::Number(n)=>{
                  if n.is_u64(){
                      where_str=where_str+ n.as_u64().unwrap().to_string().as_str()+",";
                  }else if n.is_f64(){
                      where_str=where_str+n.as_f64().unwrap().to_string().as_str()+",";
                  }else if n.is_i64(){
                      where_str=where_str+n.as_i64().unwrap().to_string().as_str()+",";
                  }else{
                      return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                  }
                }
                serde_json::Value::Null=>{
                    continue;
                }
                _ => {
                    return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                }
            }
        }
        where_str.pop();
        where_str=where_str+")";
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
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql =rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(vec![1,2,3]).borrow_mut());
    println!("{}",sql.unwrap());
}

#[test]
fn test_delete_by_map() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis =Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql =rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(r#"123123"#).borrow_mut());
    println!("{}",sql.unwrap());
}