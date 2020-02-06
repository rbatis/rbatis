use std::borrow::BorrowMut;
use std::fs;
use std::process::id;

use serde::de::{Deserialize, DeserializeOwned};
use serde::ser::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::ast::ast::Ast;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::convert::sql_value_convert;
use crate::convert::sql_value_convert::{AND, SkipType, SqlQuestionConvert, SqlValueConvert};
use crate::crud::ipage::IPage;
use crate::error::RbatisError;
use crate::example::activity::Activity;
use crate::rbatis::Rbatis;
use crate::utils::join_in::json_join;
use crate::utils::string_util::count_string_num;

impl Rbatis {
    ///普通查询
    pub fn select<T>(&mut self, id: &str, mapper_name: &str, env: &Value) -> Result<T, RbatisError> where T: DeserializeOwned {
        let mut arg_array = vec![];
        let mut arg = env.clone();
        let (sql, _) = self.create_sql_select(mapper_name, &mut arg, &mut arg_array)?;
        return self.raw_sql_prepare(id, sql.as_str(), &mut arg_array);
    }

    ///分页查询
    pub fn select_page<T>(&mut self, id: &str, mapper_name: &str, arg: &Value, ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: Serialize + DeserializeOwned + Clone {
        let mut arg_array = vec![];
        //do select
        let mut new_arg = json_join(&arg, "ipage", ipage)?;
        let (records, w) = self.eval_select_return_where(id, mapper_name, &mut new_arg, &mut arg_array)?;
        let mut result = ipage.clone();
        result.set_records(records);

        //do count
        let result_map_node = self.get_result_map_node(mapper_name)?;
        let count_sql = self.do_count_by_templete(&mut new_arg, &result_map_node, w.as_str())?;

        let total: i64 = self.raw_sql_prepare(id, count_sql.as_str(), &mut arg_array)?;
        result.set_total(total);
        return Result::Ok(result);
    }

    /// 根据mapper 自定义内容 分页查询， 只需写一个查询内容，不需要添加 count函数
    pub fn select_page_by_mapper<T>(&mut self, id: &str, mapper_name: &str, mapper_id: &str, env: &Value, ipage: &IPage<T>) -> Result<IPage<T>, RbatisError> where T: Serialize + DeserializeOwned + Clone {
        let mut arg_array = vec![];
        let mut arg = env.clone();
        let mut new_arg = json_join(&mut arg, "ipage", ipage)?;
        //select redords
        let mapper_opt = self.mapper_map.get_mut(&mapper_name.to_string());
        if mapper_opt.is_none() {
            return Result::Err(RbatisError::from("[rbatis] find mapper fail,name:'".to_string() + mapper_name + "'"));
        }
        let mapper_name_id = mapper_name.to_string() + "." + mapper_id;
        let node = mapper_opt.unwrap().get_mut(mapper_id);
        if node.is_none() {
            return Result::Err(RbatisError::from("[rbatis] no method find in : ".to_string() + mapper_name_id.as_str()));
        }
        let mapper_func = node.unwrap();
        let sql_string = mapper_func.eval(&mut new_arg, &mut self.engine, &mut arg_array)?;
        //create where str
        let mut where_befer_string = "".to_string();
        let mut where_string = "".to_string();
        if sql_string.contains("where") {
            let wheres: Vec<&str> = sql_string.split("where").collect();
            if wheres.len() > 2 {
                return Result::Err(RbatisError::from("[rbatis] find 'where' repeated > 2 time,name:'".to_string() + mapper_name_id.as_str()));
            }
            where_string = wheres[1].to_string();
            where_befer_string = wheres[0].to_string();
        } else if sql_string.contains("WHERE") {
            let wheres: Vec<&str> = sql_string.split("WHERE").collect();
            if wheres.len() > 2 {
                return Result::Err(RbatisError::from("[rbatis] find 'WHERE' repeated > 2 time,name:'".to_string() + mapper_name_id.as_str()));
            }
            where_string = wheres[1].to_string();
            where_befer_string = wheres[0].to_string();
        } else {
            where_befer_string = sql_string;
        }
        //insert delete_flag
        let mut append_limit_where_string = where_string.clone();
        if !where_string.contains(" limit ") && !where_string.contains("LIMIT") {
            append_limit_where_string = append_limit_where_string + " LIMIT " + ((ipage.current - 1) * ipage.size).to_string().as_str() + "," + ipage.size.to_string().as_str();
        }
        let query_sql = where_befer_string + " WHERE " + append_limit_where_string.as_str();

        let records: Vec<T> = self.raw_sql_prepare(id, query_sql.as_str(), &mut arg_array)?;
        let mut result = ipage.clone();
        result.set_records(records);

        //do count
        let result_map_node = self.get_result_map_node(mapper_name)?;
        let mut count_sql = "select count(1) from #{table} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err(RbatisError::from("[rbatis]  can not find table defin in <result_map>!".to_string()));
        }
        count_sql = count_sql.replace("#{table}", result_map_node.table.as_ref().unwrap());

        //sub string limit
        let limit_opt = where_string.rfind(" limit ");
        if limit_opt.is_some() {
            where_string = where_string[0..limit_opt.unwrap()].to_string();
        }
        count_sql = count_sql.replace("#{where}", where_string.as_str());
        let total = self.raw_sql_prepare(id, count_sql.as_str(), &mut arg_array)?;
        result.set_total(total);
        return Result::Ok(result);
    }


    fn eval_select_return_where<T>(&mut self, id: &str, mapper_name: &str, arg: &mut Value, arg_array: &mut Vec<Value>) -> Result<(T, String), RbatisError> where T: DeserializeOwned {
        let (sql, w) = self.create_sql_select(mapper_name, arg, arg_array)?;
        let data: T = self.raw_sql_prepare(id, sql.as_str(), arg_array)?;
        return Result::Ok((data, w));
    }

    fn create_sql_select(&self, mapper_name: &str, arg: &mut Value, arg_arr: &mut Vec<Value>) -> Result<(String, String), RbatisError> {
        let result_map_node = self.get_result_map_node(mapper_name)?;
        return match arg {
            serde_json::Value::Null => {
                Result::Err(RbatisError::from("[rbatis] arg is null value".to_string()))
            }
            serde_json::Value::String(_) | serde_json::Value::Number(_) => {
                let ipage_opt: Option<IPage<Value>> = None;
                let where_str = "id = ".to_string() + arg.to_sql_question(SkipType::Null, AND, ",", arg_arr).as_str();
                Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &ipage_opt)?)
            }
            serde_json::Value::Array(_) => {
                let ipage_opt: Option<IPage<Value>> = None;
                let where_str = "id in ".to_string() + arg.to_sql_question(SkipType::Null, AND, ",", arg_arr).as_str();
                Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &ipage_opt)?)
            }
            serde_json::Value::Object(map) => {
                let mut ipage_opt = None;
                let ipage_value_opt = map.get("ipage");
                if ipage_value_opt.is_some() {
                    let ipage_value = ipage_value_opt.unwrap();
                    if !ipage_value.is_null() {
                        let ipage: Result<IPage<Value>, serde_json::Error> = serde_json::from_value(ipage_value.clone());
                        if ipage.is_err() {
                            return Result::Err(RbatisError::from("[rbatis] ".to_string() + ipage.err().unwrap().to_string().as_str()));
                        }
                        ipage_opt = Some(ipage.unwrap());
                    }
                }
                let where_str = arg.to_sql_question(SkipType::None, AND, ",", arg_arr);
                Result::Ok(self.do_select_by_templete(arg, &result_map_node, where_str.as_str(), &ipage_opt)?)
            }
            _ => {
                Result::Err(RbatisError::from("[rbatis] not support arg type value in select(): ".to_string() + arg.to_sql_value_def(true).as_str()))
            }
        };
    }


    fn create_sql_count(&mut self, mapper_name: &str, arg: &mut Value, arg_arr: &mut Vec<Value>) -> Result<String, RbatisError> {
        let result_map_node = self.get_result_map_node(mapper_name)?;
        match arg {
            serde_json::Value::Null => {
                return Result::Err(RbatisError::from("[rbatis] arg is null value"));
            }
            serde_json::Value::String(_) | serde_json::Value::Number(_) => {
                let where_str = "id = ".to_string() + arg.to_sql_question(SkipType::Null, AND, ",", arg_arr).as_str();
                return Result::Ok(self.do_count_by_templete(arg, &result_map_node, where_str.as_str())?);
            }
            serde_json::Value::Array(_) => {
                let where_str = "id in ".to_string() + arg.to_sql_question(SkipType::Null, AND, ",", arg_arr).as_str();
                return Result::Ok(self.do_count_by_templete(arg, &result_map_node, where_str.as_str())?);
            }
            serde_json::Value::Object(map) => {
                let mut ipage_opt = None;
                let ipage_value_opt = map.get("ipage");
                if ipage_value_opt.is_some() {
                    let ipage_value = ipage_value_opt.unwrap();
                    if !ipage_value.is_null() {
                        let ipage: Result<IPage<Value>, serde_json::Error> = serde_json::from_value(ipage_value.clone());
                        if ipage.is_err() {
                            return Result::Err(RbatisError::from("[rbatis] ".to_string() + ipage.err().unwrap().to_string().as_str()));
                        }
                        ipage_opt = Some(ipage.unwrap());
                    }
                }
                let where_str = arg.to_sql_question(SkipType::None, AND, ",", arg_arr);
                return Result::Ok(self.do_count_by_templete(arg, &result_map_node, where_str.as_str())?);
            }
            _ => {
                return Result::Err(RbatisError::from("[rbatis] not support arg type value in select(): ".to_string() + arg.to_sql_value_def(true).as_str()));
            }
        }
    }


    /// return 结果/where sql
    fn do_select_by_templete<T>(&self, env: &mut Value, result_map_node: &ResultMapNode, where_str: &str, ipage_opt: &Option<IPage<T>>) -> Result<(String, String), RbatisError> where T: Serialize + DeserializeOwned + Clone {
        let mut sql = "select * from #{table} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err(RbatisError::from("[rbatis]  can not find table defin in <result_map>!".to_string()));
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());

        //replace where
        let mut where_string = where_str.to_string();
        where_string.trim();
        //delete node
        if result_map_node.delete_node.is_some() && !where_string.contains(result_map_node.delete_node.as_ref().unwrap().column.as_str()) {
            if where_string.is_empty() {
                where_string = result_map_node.delete_node.as_ref().unwrap().column.clone() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str() + where_string.as_str();
            } else {
                where_string = result_map_node.delete_node.as_ref().unwrap().column.clone() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str() + AND + where_string.as_str();
            }
        }
        //replace where
        sql = sql.replace("#{where}", where_string.as_str());

        if ipage_opt.is_some() {
            let ipage = ipage_opt.as_ref().unwrap();
            sql = sql + " LIMIT " + ((ipage.current - 1) * ipage.size).to_string().as_str() + "," + ipage.size.to_string().as_str();
        }
        return Result::Ok((sql, where_string));
    }

    /// return 结果/where sql
    fn do_count_by_templete(&self, env: &mut Value, result_map_node: &ResultMapNode, where_str: &str) -> Result<String, RbatisError> {
        let mut sql = "select count(1) from #{table} where #{where}".to_string();
        //replace table
        if result_map_node.table.is_none() {
            return Result::Err(RbatisError::from("[rbatis]  can not find table defin in <result_map>!".to_string()));
        }
        sql = sql.replace("#{table}", result_map_node.table.as_ref().unwrap());

        //replace where
        let mut where_string = where_str.to_string();
        where_string.trim();
        //delete node
        if result_map_node.delete_node.is_some() && !where_string.contains(result_map_node.delete_node.as_ref().unwrap().column.as_str()) {
            if where_string.is_empty() {
                where_string = result_map_node.delete_node.as_ref().unwrap().column.clone() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str() + where_string.as_str();
            } else {
                where_string = result_map_node.delete_node.as_ref().unwrap().column.clone() + " = " + result_map_node.delete_node.as_ref().unwrap().logic_undelete.as_str() + AND + where_string.as_str();
            }
        }
        //replace where
        sql = sql.replace("#{where}", where_string.as_str());
        return Result::Ok(sql);
    }
}

#[test]
fn test_select_by_id() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    let mut arg_array = vec![];
    let (sql, _) = rbatis.create_sql_select("Example_ActivityMapper.xml", serde_json::json!("1").borrow_mut(), &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));

    assert_eq!(arg_array.len(), count_string_num(&sql, '?'));
}

#[test]
fn test_select_by_ids() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    let mut arg_array = vec![];
    let (sql, _) = rbatis.create_sql_select("Example_ActivityMapper.xml", serde_json::json!(vec![1,2,3]).borrow_mut(), &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));

    assert_eq!(arg_array.len(), count_string_num(&sql, '?'));
}

#[test]
fn test_select_by_map() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    let mut arg_array = vec![];
    let (sql, _) = rbatis.create_sql_select("Example_ActivityMapper.xml", serde_json::json!({
     "arg": 2,
     "delete_flag":1,
     "number_arr":vec![1,2,3],
     "string_arr":vec!["1","2","3"]
    }).borrow_mut(), &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));

    assert_eq!(arg_array.len(), count_string_num(&sql, '?'));
}

#[test]
fn test_select_by_id_page() {
    let mut rbatis = Rbatis::new();
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    let mut arg_array = vec![];
    let act = Activity {
        id: None,
        name: Some("新人专享".to_string()),
        pc_link: None,
        h5_link: None,
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: Some(1),
        remark: None,
        create_time: None,
        version: None,
        delete_flag: Some(1),
    };
    let ipage: IPage<Value> = IPage::new(1, 20);
    let arg = json_join(&act, "ipage", ipage).unwrap();
    let (sql, w) = rbatis.create_sql_select("Example_ActivityMapper.xml", serde_json::to_value(arg).unwrap().borrow_mut(), &mut arg_array).unwrap();
    println!("{}", sql);
    println!("{}", json!(arg_array));

    assert_eq!(arg_array.len(), count_string_num(&sql, '?'));
}