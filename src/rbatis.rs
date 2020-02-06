use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::process::exit;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use log4rs::init_file;
use serde::{de, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{Number, Value};
use serde_json::de::ParserNumber;
use serde_json::json;
use serde_json::ser::State::Rest;
use uuid::Uuid;

use crate::ast::ast::Ast;
use crate::ast::lang::py::Py;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::node::{do_child_nodes, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::ast::node::result_map_node::ResultMapNode;
use crate::ast::node::string_node::StringNode;
use crate::crud::ipage::IPage;
use crate::db_config::DBConfig;
use crate::decode::rdbc_driver_decoder::decode_result_set;
use crate::engine::runtime::RbatisEngine;
use crate::error::RbatisError;
use crate::local_session::LocalSession;
use crate::session_factory::{SessionFactory, SessionFactoryCached};
use crate::tx::propagation::Propagation;
use crate::utils::{driver_util, rdbc_util};
use crate::utils::rdbc_util::to_rdbc_values;
use crate::utils::xml_loader::load_xml;

lazy_static! {
  static ref RBATIS: Mutex<Rbatis> = Mutex::new(Rbatis::new());
}

///使用 lazy_static 获取的单例
pub fn singleton() -> MutexGuard<'static, Rbatis> {
    return RBATIS.lock().unwrap();
}

pub fn eval_sql<T>(id: &str, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned {
    return singleton().raw_sql(id, eval_sql);
}


pub struct Rbatis {
    pub id: String,
    //动态sql运算节点集合
    pub mapper_map: HashMap<String, HashMap<String, NodeType>>,
    //动态sql节点配置
    pub engine: RbatisEngine,
    //数据库驱动
    pub db_driver: String,
    //session工厂
    pub session_factory: SessionFactoryCached,
    //允许日志输出，禁用此项可减少IO,提高性能
    pub enable_log: bool,
    //true异步模式，false线程模式
    pub async_mode: bool,
}

impl Rbatis {
    pub fn new() -> Self {
        return Self {
            id: Uuid::new_v4().to_string(),
            mapper_map: HashMap::new(),
            engine: RbatisEngine::new(),
            db_driver: "".to_string(),
            session_factory: SessionFactoryCached::new(false),
            enable_log: true,
            async_mode: false,
        };
    }

    pub fn new_factory() -> Box<dyn SessionFactory> {
        return Box::new(SessionFactoryCached::new(false));
    }


    pub fn set_enable_log(&mut self, arg: bool) {
        self.enable_log = arg;
    }

    ///加载xml数据
    /// rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    pub fn load_xml(&mut self, key: String, content: String) {
        if self.enable_log {
            info!("===========load {}==============\n{}\n================ end {}===============", key, content, key);
        }
        self.mapper_map.insert(key, crate::ast::lang::xml::parser(content, &self.engine));
    }


    /// 设置数据库默认url，如果失败返回错误信息
    ///  let url = "mysql://root:TEST@localhost:3306/test";
    pub fn load_db_url(&mut self, url: &str) -> Option<RbatisError> {
        let db_config_opt = DBConfig::new(url.to_string());
        if db_config_opt.is_ok() {
            self.db_driver = url.to_string();
            return Option::None;
        } else {
            let e = db_config_opt.err().unwrap();
            if self.enable_log {
                error!("{}", "[rbatis] link db fail:".to_string() + e.description());
            }
            return Option::Some(e);
        }
    }


    pub fn begin(&mut self, id: &str, propagation_type: Propagation) -> Result<&mut LocalSession, RbatisError> {
        self.check_driver()?;
        let session = self.session_factory.get_thread_session(&id.to_string(), self.db_driver.as_str())?;
        session.begin(propagation_type)?;
        return Result::Ok(session);
    }

    pub fn rollback<'a>(&mut self, id: &'a str) -> Result<&'a str, RbatisError> {
        self.check_driver()?;
        self.session_factory.get_thread_session(&id.to_string(), self.db_driver.as_str())?.rollback()?;
        self.session_factory.data.remove(&id.to_string());
        return Result::Ok(id);
    }

    pub fn commit<'a>(&mut self, id: &'a str) -> Result<&'a str, RbatisError> {
        self.check_driver()?;
        self.session_factory.get_thread_session(&id.to_string(), self.db_driver.as_str())?.commit()?;
        self.session_factory.data.remove(&id.to_string());
        return Result::Ok(id);
    }


    /// 执行py sql到数据库，例如:
    ///    Result中结果可以为serde_json::Value，Vec，Array,Slice,LinkedList,Map,i32
    ///
    ///    let data: Vec<Activity> = rbatis.unwrap().py_sql("Example_ActivityMapper.xml", &mut json!({
    ///       "name":"新人专享",
    ///       "delete_flag": 1,
    ///    }), "
    ///    SELECT * FROM biz_activity WHERE delete_flag = 1
    ///    if name != null:
    ///      AND name like #{name+'%'}
    ///    ").unwrap();
    ///    println!("[rbatis] result==>  {:?}", data);
    ///
    pub fn py_sql<T>(&mut self, id: &str, mapper_name: &str, env: &Value, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        let pys = Py::parser_by_cache(eval_sql)?;
        let mut arg_array = vec![];
        let mut new_env = env.clone();
        let raw_sql = do_child_nodes(&pys, &mut new_env, &mut self.engine, &mut arg_array)?;
        return self.raw_sql_prepare(id, raw_sql.as_str(), &mut arg_array);
    }


    ///执行sql到数据库，例如:
    ///    Result中结果可以为serde_json::Value，Vec，Array,Slice,LinkedList,Map,i32
    ///
    ///    let data_opt: Result<serde_json::Value, RbatisError> = rbatis.raw_sql( "select * from table", &mut json!({
    ///       "name":null,
    ///       "startTime":null,
    ///       "endTime":null,
    ///       "page":null,
    ///       "size":null,
    ///    }));
    ///
    pub fn raw_sql<T>(&mut self, id: &str, eval_sql: &str) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        let mut arg_array = vec![];
        return self.raw_sql_prepare(id, eval_sql, &mut arg_array);
    }

    ///执行
    /// arg_array: 执行后 需要替换的参数数据
    /// return ：替换参数为 ？ 后的sql
    pub fn raw_sql_prepare<T>(&mut self, id: &str, eval_sql: &str, arg_array: &mut Vec<Value>) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        let mut sql = eval_sql;
        sql = sql.trim();
        if sql.is_empty() {
            return Result::Err(RbatisError::from("[rbatis] sql can not be empty!".to_string()));
        }
        let params = to_rdbc_values(arg_array);
        self.check_driver()?;
        let is_select = sql.starts_with("select") || sql.starts_with("SELECT") || sql.starts_with("Select");
        if is_select {
            //select
            let session = self.session_factory.get_thread_session(&id.to_string(), self.db_driver.as_str())?;
            return session.query(sql, &params);
        } else {
            //exec
            let session = self.session_factory.get_thread_session(&id.to_string(), self.db_driver.as_str())?;
            let affected_rows = session.exec(sql, &params)?;
            let r = serde_json::from_value(serde_json::Value::Number(serde_json::Number::from(ParserNumber::U64(affected_rows))));
            if r.is_err() {
                return Result::Err(RbatisError::from("[rbatis] exec fail:".to_string() + id + r.err().unwrap().to_string().as_str()));
            }
            if self.enable_log {
                info!(" Affected: <== {}: {}", id, affected_rows.to_string().as_str());
            }
            return Result::Ok(r.unwrap());
        }
    }

    ///执行sql到数据库，例如
    ///
    ///    let data_opt: Result<serde_json::Value, RbatisError> = rbatis.mapper("Example_ActivityMapper.xml".to_string(), "select_by_condition", &mut json!({
    ///       "name":null,
    ///       "startTime":null,
    ///       "endTime":null,
    ///       "page":null,
    ///       "size":null,
    ///    }));
    ///
    pub fn mapper<T>(&mut self, id: &str, mapper_name: &str, mapper_id: &str, env: &Value, arg_array: &mut Vec<Value>) -> Result<T, RbatisError> where T: de::DeserializeOwned {
        let mut arg = env.clone();
        let mapper_opt = self.mapper_map.get(&mapper_name.to_string());
        if mapper_opt.is_none() {
            return Result::Err(RbatisError::from("[rbatis] find mapper fail,name:'".to_string() + mapper_name + "'"));
        }
        let mapper_name_id = mapper_name.to_string() + "." + mapper_id;

        let node = mapper_opt.unwrap().get(mapper_id);
        if node.is_none() {
            return Result::Err(RbatisError::from("[rbatis] no method find in : ".to_string() + mapper_name_id.as_str()));
        }
        let mapper_func = node.unwrap();
        let sql_string = mapper_func.eval(&mut arg, &mut self.engine, arg_array)?;
        let sql = sql_string.as_str();
        return self.raw_sql_prepare(id, sql, arg_array);
    }


    ///打印内容
    pub fn print(&self) -> String {
        let mut result = String::new();
        for (key, node_types) in &self.mapper_map {
            for (key, node) in node_types {
                let data = node.print(0);
                let data_str = data.as_str();
                result += data_str;
                println!("\n{}", data_str);
            }
        }
        return result;
    }

    /// find result map config
    pub fn get_result_map_node(&self, mapper_name: &str) -> Result<ResultMapNode, RbatisError> {
        let result_map_opt = self.mapper_map.get(mapper_name);
        if result_map_opt.is_none() {
            return Result::Err(RbatisError::from("[rbatis]  can not be find ".to_string() + mapper_name));
        }
        let result_map = result_map_opt.unwrap();
        let base_result_map_opt = result_map.get("BaseResultMap");
        if base_result_map_opt.is_some() {
            let base_result_map = base_result_map_opt.unwrap().to_result_map_node();
            if base_result_map.is_some() {
                return Result::Ok(base_result_map.unwrap());
            }
        }
        return Result::Err(RbatisError::from("[rbatis]  can not be find ".to_string() + mapper_name));
    }

    fn check_driver(&self) -> Result<bool, RbatisError> {
        if self.db_driver.is_empty() {
            return Result::Err(RbatisError::from("[rbatis] no DataBase driver find!"));
        }
        return Ok(true);
    }
}