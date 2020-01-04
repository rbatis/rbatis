use std::borrow::{BorrowMut, Borrow};
use std::fs;

use serde_json::{Value, Map};

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::core::rbatis::Rbatis;
use crate::engine::node::NodeType;
use serde_json::value::Value::Number;
use crate::convert::sql_value_convert::SqlValueConvert;

const AND: &'static str = " and ";

impl Rbatis{
    pub fn delete(&mut self, mapper_name: &str, id: &str, arg: &mut Value)-> Result<String, String>{
        if arg.is_null() {
            return Result::Err("[rbatis] arg is null value".to_string());
        }
        let result_map_node=self.get_result_map_node(mapper_name,id)?;
        match arg {
            serde_json::Value::String(str)=>{
                //delete by id
                let where_str=self.do_delete_by_id_where(arg, &result_map_node)?;
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Array(arr)=>{
                //delete by ids
                let mut string_arr=vec![];
                for item in arr {
                    match item {
                          serde_json::Value::String(_)
                        | serde_json::Value::Number(_)=>{
                            string_arr.push(item.clone());
                        }
                        _ => {
                            return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                        }
                    }
                }
                let where_str=self.do_delete_by_ids_where(arg, &result_map_node, string_arr)?;
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Object(map)=>{
                let  c=map.clone();
                let where_str=self.do_delete_by_map_where(arg, &result_map_node,&c )?;
                return self.do_delete_by(arg,&result_map_node,where_str.as_str());
            }
            serde_json::Value::Null=>{
                return Result::Err("[rbatis] delete arg type can not be null!".to_string());
            }
            _ => {
                return Result::Err("[rbatis] not support arg type".to_string());
            }
        };
    }


    ///基本删除语句框架
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

    ///where delete by id
    fn do_delete_by_id_where(&mut self, env: &mut Value, result_map_node:&ResultMapNode) -> Result<String, String>{
        //replace where
        let mut where_str = "id = ".to_string() + env.to_sql().as_str();
        return Result::Ok(where_str);
    }
    ///where delete by ids
    fn do_delete_by_ids_where(&mut self, env: &mut Value, result_map_node:&ResultMapNode, arr:Vec<Value>) -> Result<String, String>{
        //replace where
        let mut where_str = "id in (".to_string();
        for x in &arr {
            match x{
                serde_json::Value::String(s)=>{
                    where_str=where_str+x.to_sql().as_str()+",";
                }
                serde_json::Value::Number(n)=>{
                    where_str=where_str+ x.to_sql().as_str()+",";
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
        return Result::Ok(where_str);
    }


    ///where delete by map,support  string,number,vec
    fn do_delete_by_map_where(&mut self, env: &mut Value, result_map_node:&ResultMapNode,arg_map:&Map<String,Value>)-> Result<String, String>{
        let mut where_str="".to_string();
        let len=arg_map.len();
        for (key,value) in arg_map{
            match value{
                Value::String(s)=>{
                    where_str=where_str+key.as_str()+" = "+value.to_sql().as_str() +" and "
                }
                Value::Number(n)=>{
                    where_str=where_str+key.as_str()+" = "+value.to_sql().as_str() +" and "
                }
                Value::Array(arr)=>{
                    where_str=where_str+key.as_str()+" in "+value.to_sql().as_str() +" and "
                }
                _ => {
                    return Result::Err("[rbatis] not support arg! delete by arr,arr must be string array or number array!".to_string());
                }
            }
        }
        if len>0{
            for _ in 0..AND.len() {
                where_str.pop();
            }
        }
        return Result::Ok(where_str);
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

    let sql =rbatis.delete("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!({
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"]
    }).borrow_mut());
    println!("{}",sql.unwrap());
}