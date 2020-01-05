use std::borrow::BorrowMut;
use std::fs;

use serde_json::Value;

use crate::ast::xml::result_map_node::ResultMapNode;
use crate::convert::sql_value_convert;
use crate::convert::sql_value_convert::SqlValueConvert;
use crate::core::rbatis::Rbatis;
use crate::crud::ipage::IPage;
use crate::example::activity::Activity;

impl Rbatis {
    pub fn select(&mut self, mapper_name: &str, id: &str, arg: &mut Value) -> Result<String, String> {
        //TODO select by page
        let result_map_node = self.get_result_map_node(mapper_name, id)?;
        match arg {
            serde_json::Value::Null => {
                return Result::Err("[rbatis] arg is null value".to_string());
            }
            serde_json::Value::String(_) | serde_json::Value::Number(_) => {
                let mut where_str = "id = ".to_string() + arg.to_sql_value_skip("null").as_str();
                return Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &None)?);
            }
            serde_json::Value::Array(_) => {
                let mut where_str = "id in ".to_string() + arg.to_sql_value_skip("null").as_str();
                return Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &None)?);
            }
            serde_json::Value::Object(map) => {
                let mut ipage_opt = None;
                let ipage_value_opt = map.get("ipage");
                if ipage_value_opt.is_some() {
                    let ipage_value = ipage_value_opt.unwrap();
                    if !ipage_value.is_null() {
                        let ipage: Result<IPage<Value>,serde_json::Error> = serde_json::from_value(ipage_value.clone());
                        if ipage.is_err(){
                            return Result::Err("[rbatis] ".to_string()+ipage.err().unwrap().to_string().as_str());
                        }
                        ipage_opt = Some(ipage.unwrap());
                    }
                }
                let mut where_str = arg.to_sql_value_skip("null");
                return Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &ipage_opt)?);
            }
            _ => {
                return Result::Err("[rbatis] not support arg type value in select(): ".to_string() + arg.to_sql_value_def().as_str());
            }
        }
        return Result::Err("[rbatis] eval select crud fail".to_string());
    }

    fn do_select_by_templete(&mut self, env: &mut Value, result_map_node: &ResultMapNode, where_str: &str, ipage_opt: &Option<IPage<Value>>) -> Result<String, String> {
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

        if ipage_opt.is_some(){
            sql = sql + " LIMIT "+ipage_opt.as_ref().unwrap().current.to_string().as_str()+","+ipage_opt.as_ref().unwrap().size.to_string().as_str();
        }
        return Result::Ok(sql);
    }
}

#[test]
fn test_select_by_id() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql = rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!("1").borrow_mut());
    println!("{}", sql.unwrap());
}

#[test]
fn test_select_by_ids() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql = rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!(vec![1,2,3]).borrow_mut());
    println!("{}", sql.unwrap());
}

#[test]
fn test_select_by_map() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let sql = rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::json!({
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"]
    }).borrow_mut());
    println!("{}", sql.unwrap());
}

#[test]
fn test_select_by_id_page() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据

    let act=Activity{
        id: None,
        name: Some("新人专享".to_string()),
        pc_link: None,
        h5_link: None,
        remark: None,
        create_time: None,
        version: None,
        delete_flag: Some(1)
    };
    let mut arg= serde_json::to_value(act).unwrap();
    let ipage:IPage<Value>=IPage::new(1,20);
    arg.as_object_mut().unwrap().insert("ipage".to_string(),serde_json::to_value(ipage).unwrap());
    let sql = rbatis.select("Example_ActivityMapper.xml", "BaseResultMap", serde_json::to_value(arg).unwrap().borrow_mut());
    println!("{}", sql.unwrap());
}