use std::any::Any;
use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Mutex;

use log::{error, info, warn};
use log4rs::init_file;
use serde::de;
use serde_json::{Number, Value};
use serde_json::json;
use serde_json::ser::State::Rest;

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::bind_node::BindNode;
use crate::ast::xml::node::{loop_decode_xml, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::result_map_node::ResultMapNode;
use crate::ast::xml::string_node::StringNode;
use crate::db_config::DBConfig;
use crate::node_type_map_factory::create_node_type_map;
use crate::decode::rdbc_driver_decoder::decode_result_set;
use crate::utils::{driver_util, rdbc_util};
use crate::utils::xml_loader::load_xml;
use crate::utils::rdbc_util::to_rdbc_values;
use serde_json::de::ParserNumber;
use crate::session::Session;
use uuid::Uuid;
use crate::tx::propagation::Propagation;
use crate::conn_factory::{ConnFactory, ConnFactoryImpl};
use std::thread;

pub struct Rbatis {
    pub id :String,
    //动态sql运算节点集合
    pub mapper_map: HashMap<String, HashMap<String, NodeType>>,
    //动态sql节点配置
    pub holder: ConfigHolder,
    //路由配置
    pub db_driver_map: HashMap<String, String>,
    pub router_func: fn(id: &str) -> String,
    //连接工厂
    pub conn_factory: Box<dyn ConnFactory>,
    //允许日志输出，禁用此项可减少IO,提高性能
    pub enable_log: bool,
    //true异步模式，false线程模式
    pub async_mode: bool,
}


impl Rbatis {
    pub fn new() -> Rbatis {
        return Rbatis {
            id: Uuid::new_v4().to_string(),
            mapper_map: HashMap::new(),
            holder: ConfigHolder::new(),
            db_driver_map: HashMap::new(),
            conn_factory: Box::new(ConnFactoryImpl::new(true)),
            router_func: |id| -> String{
                //加载默认配置，key=""
                return "".to_string();
            },
            enable_log: true,
            async_mode: false,
        };
    }


    pub fn set_enable_log(&mut self, arg: bool) {
        self.enable_log = arg;
    }

    ///加载xml数据
    /// rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    pub fn load_xml(&mut self, key: String, content: String) {
        if self.enable_log {
            info!("[rbatis]===========load {}==============\n{}\n================ end {}===============", key, content, key);
        }
        self.mapper_map.insert(key, create_node_type_map(content, &self.holder));
    }


    /// 设置数据库默认url，如果失败返回错误信息
    ///  let url = "mysql://root:TEST@localhost:3306/test";
    ///  rbatis.load_db_url("".to_string(), url.to_string());//name 为空，则默认数据库
    pub fn load_db_url(&mut self, key: &str, url: &str) -> Option<String> {
        let db_config_opt = DBConfig::new(url.to_string());
        if db_config_opt.is_ok() {
            self.db_driver_map.insert(key.to_string(), url.to_string());
            return Option::None;
        } else {
            let e = db_config_opt.err().unwrap();
            if self.enable_log {
                error!("{}", "[rbatis] link db fail:".to_string() + e.as_str());
            }
            return Option::Some(e);
        }
    }


    ///执行sql到数据库，例如:
    ///    Result中结果可以为serde_json::Value，Vec，Array,Slice,LinkedList,Map,i32
    ///
    ///    let data_opt: Result<serde_json::Value, String> = rbatis.eval( "select * from table", &mut json!({
    ///       "name":null,
    ///       "startTime":null,
    ///       "endTime":null,
    ///       "page":null,
    ///       "size":null,
    ///    }));
    ///
    pub fn eval_sql<T>(&mut self, eval_sql: &str) -> Result<T, String> where T: de::DeserializeOwned {
        let mut sql = eval_sql;
        sql = sql.trim();
        if sql.is_empty() {
            return Result::Err("[rbatis] sql can not be empty!".to_string());
        }
        let is_select = sql.starts_with("select") || sql.starts_with("SELECT");
        let mut arg_array = vec![];
        return self.eval_raw("eval_sql", eval_sql, is_select, &mut arg_array);
    }

    pub fn begin(&mut self,id: &str) -> Result<u64, String>{
        let data=self.eval_sql_source(id, "begin;")?;
        return Result::Ok(data);
    }
    pub fn commit(&mut self,id: &str) -> Result<u64, String>{
        let data=self.eval_sql_source(id, "commit;")?;
        return Result::Ok(data);
    }
    pub fn rollback(&mut self,id: &str) -> Result<u64, String>{
        let data=self.eval_sql_source(id, "rollback;")?;
        return Result::Ok(data);
    }

    ///执行无编译的原生sql
    pub fn eval_sql_source(&mut self, id: &str, sql:&str) -> Result<u64, String>{
        let key = (self.router_func)(id);
        let db_conf_opt = self.db_driver_map.get(key.as_str());
        if db_conf_opt.is_none() {
            return Result::Err("[rbatis] no DBConfig:".to_string() + key.as_str() + " find!");
        }
        let driver = db_conf_opt.unwrap();
        let thread_id=thread::current().id();
        let conn = self.conn_factory.get_thread_conn(thread_id,driver.as_str())?;
        let create_result = conn.create(sql);
        if create_result.is_err() {
            return Result::Err("[rbatis] exec fail:".to_string()  + format!("{:?}", create_result.err().unwrap()).as_str());
        }
        let exec_result = create_result.unwrap().execute_update(&[]);
        if exec_result.is_err() {
            return Result::Err("[rbatis] exec fail:".to_string()  + format!("{:?}", exec_result.err().unwrap()).as_str());
        }
        return Result::Ok(exec_result.unwrap())
    }

    ///执行
    /// arg_array: 执行后 需要替换的参数数据
    /// return ：替换参数为 ？ 后的sql
    pub fn eval_raw<T>(&mut self, id: &str, eval_sql: &str, is_select: bool, arg_array: &mut Vec<Value>) -> Result<T, String> where T: de::DeserializeOwned {
        let mut sql = eval_sql;
        sql = sql.trim();
        if sql.is_empty() {
            return Result::Err("[rbatis] sql can not be empty!".to_string());
        }
        let params = to_rdbc_values(arg_array);
        if self.enable_log {
            if is_select {
                info!("[rbatis] Query: ==>  {}: {}", id, sql);
                info!("[rbatis]  Args: ==>  {}: {}", id, rdbc_util::rdbc_vec_to_string(&params));
            } else {
                info!("[rbatis]  Exec:  ==>  {}: {}", id, sql);
                info!("[rbatis]  Args:  ==>  {}: {}", id, rdbc_util::rdbc_vec_to_string(&params));
            }
        }
        let key = (self.router_func)(id);
        let db_conf_opt = self.db_driver_map.get(key.as_str());
        if db_conf_opt.is_none() {
            return Result::Err("[rbatis] no DBConfig:".to_string() + key.as_str() + " find!");
        }
        let driver = db_conf_opt.unwrap();
        let thread_id=thread::current().id();
        let conn = self.conn_factory.get_thread_conn(thread_id,driver.as_str())?;
        if is_select {
            //select
            let create_result = conn.prepare(sql);
            if create_result.is_err() {
                return Result::Err("[rbatis] select fail:".to_string() + id + format!("{:?}", create_result.err().unwrap()).as_str());
            }
            let mut create_statement = create_result.unwrap();
            let exec_result = create_statement.execute_query(&params);
            if exec_result.is_err() {
                return Result::Err("[rbatis] select fail:".to_string() + id + format!("{:?}", exec_result.err().unwrap()).as_str());
            }
            let (result, decoded_num) = decode_result_set(exec_result.unwrap().as_mut());
            if self.enable_log {
                info!("[rbatis] Total: <== {}: {}", id, decoded_num.to_string().as_str());
            }
            return result;
        } else {
            //exec
            let create_result = conn.prepare(sql);
            if create_result.is_err() {
                return Result::Err("[rbatis] exec fail:".to_string() + id + format!("{:?}", create_result.err().unwrap()).as_str());
            }
            let exec_result = create_result.unwrap().execute_update(&params);
            if exec_result.is_err() {
                return Result::Err("[rbatis] exec fail:".to_string() + id + format!("{:?}", exec_result.err().unwrap()).as_str());
            }
            let affected_rows = exec_result.unwrap();
            let r = serde_json::from_value( serde_json::Value::Number(serde_json::Number::from(ParserNumber::U64(affected_rows))));
            if r.is_err() {
                return Result::Err("[rbatis] exec fail:".to_string() + id + r.err().unwrap().to_string().as_str());
            }
            if self.enable_log {
                info!("[rbatis] Affected: <== {}: {}", id, affected_rows.to_string().as_str());
            }
            return Result::Ok(r.unwrap());
        }
    }

    ///执行sql到数据库，例如
    ///
    ///    let data_opt: Result<serde_json::Value, String> = rbatis.eval("Example_ActivityMapper.xml".to_string(), "select_by_condition", &mut json!({
    ///       "name":null,
    ///       "startTime":null,
    ///       "endTime":null,
    ///       "page":null,
    ///       "size":null,
    ///    }));
    ///
    pub fn eval<T>(&mut self, mapper_name: &str, id: &str, env: &mut Value, arg_array: &mut Vec<Value>) -> Result<T, String> where T: de::DeserializeOwned {
        let mapper_opt = self.mapper_map.get(&mapper_name.to_string());
        if mapper_opt.is_none() {
            return Result::Err("[rbatis] find mapper fail,name:'".to_string() + mapper_name + "'");
        }
        let node = mapper_opt.unwrap().get(id);
        if node.is_none() {
            return Result::Err("[rbatis] find method fail,name:'".to_string() + mapper_name + id + "'");
        }
        let mapper_func = node.unwrap();
        let sql_string = mapper_func.eval(env, &mut self.holder,arg_array)?;
        let sql = sql_string.as_str();

        let sql_id = mapper_name.to_string() + "." + id;
        match &mapper_func {
            NodeType::NSelectNode(_) => {
                return self.eval_raw(sql_id.as_str(), sql, true, arg_array);
            }
            _ => {
                return self.eval_raw(sql_id.as_str(), sql, false, arg_array);
            }
        }
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
    pub fn get_result_map_node(&self, mapper_name: &str) -> Result<ResultMapNode, String> {
        let result_map_opt = self.mapper_map.get(mapper_name);
        if result_map_opt.is_none() {
            return Result::Err("[rbatis]  can not be find ".to_string() + mapper_name);
        }
        let result_map = result_map_opt.unwrap();
        let base_result_map_opt = result_map.get("BaseResultMap");
        if base_result_map_opt.is_some() {
            let base_result_map = base_result_map_opt.unwrap().to_result_map_node();
            if base_result_map.is_some() {
                return Result::Ok(base_result_map.unwrap());
            }
        }
        return Result::Err("[rbatis]  can not be find ".to_string() + mapper_name);
    }
}