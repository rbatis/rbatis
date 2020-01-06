use std::any::Any;
use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Mutex;

use log::{error, info, warn};
use log4rs::init_file;
use mysql::Conn;
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
use crate::core::conn_pool::ConnPool;
use crate::core::db_config::DBConfig;
use crate::core::node_type_map_factory::create_node_type_map;
use crate::decode::decoder::Decoder;
use crate::utils::driver_util;
use crate::utils::xml_loader::load_xml;

pub struct Rbatis {
    //动态sql运算节点集合
    pub mapper_map: HashMap<String, HashMap<String, NodeType>>,
    //动态sql节点配置
    pub holder: ConfigHolder,
    //数据库连接配置
    pub db_configs: HashMap<String, DBConfig>,
    //路由配置
    pub router_configs: HashMap<String, String>,
    //连接池
    pub conn_pool: ConnPool,

    //允许日志输出，禁用此项可减少IO,提高性能
    pub enable_log: bool,
}

impl Rbatis {
    pub fn new() -> Rbatis {
        return Rbatis {
            mapper_map: HashMap::new(),
            holder: ConfigHolder::new(),
            db_configs: HashMap::new(),
            router_configs: HashMap::new(),
            conn_pool: ConnPool::new(),
            enable_log: true,
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
    pub fn load_db_url(&mut self, name: String, url: &str) -> Option<String> {
        let db_config_opt = DBConfig::new(url.to_string());
        if db_config_opt.is_ok() {
            if name.is_empty() {
                self.db_configs.insert("".to_string(), db_config_opt.unwrap());
            } else {
                self.db_configs.insert(name, db_config_opt.unwrap());
            }
            return Option::None;
        } else {
            let e = db_config_opt.err().unwrap();
            if self.enable_log {
                error!("{}", "[rbatis] link db fail:".to_string() + e.as_str());
            }
            return Option::Some(e);
        }
    }


    /// 移除数据库url
    pub fn remove_db_url(&mut self, name: String) {
        self.db_configs.remove(&name);
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
    pub fn eval_sql<T>(&mut self, eval_sql: &str, db: &str) -> Result<T, String> where T: de::DeserializeOwned {
        let mut sql = eval_sql;
        sql = sql.trim();
        if sql.is_empty() {
            return Result::Err("[rbatis] sql can not be empty！".to_string());
        }
        let is_select = sql.starts_with("select") || sql.starts_with("SELECT");
        return self.eval_sql_raw(eval_sql, is_select, db);
    }


    pub fn eval_sql_raw<T>(&mut self, eval_sql: &str, is_select: bool, db: &str) -> Result<T, String> where T: de::DeserializeOwned {
        let mut sql = eval_sql;
        sql = sql.trim();
        if sql.is_empty() {
            return Result::Err("[rbatis] sql can not be empty！".to_string());
        }
        if self.enable_log {
            if is_select {
                info!("[rbatis] Query ==>  {}", sql);
            } else {
                info!("[rbatis] Query ==>  {}", sql);
            }
        }
        let conf_opt = self.db_configs.get(db);
        if conf_opt.is_none() {
            if self.enable_log {
                error!("{}", "[rbatis] find default database url config:".to_string() + db + " fail!");
            }
            return Result::Err("[rbatis] find default database url config:".to_string() + db + " fail!");
        }
        let conf = conf_opt.unwrap();
        let db_type = conf.db_type.as_str();
        match db_type {
            "mysql" => {
                let conn_opt = self.conn_pool.get_mysql_conn("".to_string(), conf)?;
                if is_select {
                    //select
                    let exec_result = conn_opt.unwrap().prep_exec(sql, {});
                    if exec_result.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                    }
                    let (result, decoded_num) = exec_result.unwrap().decode();
                    if self.enable_log {
                        info!("{}", "[rbatis] ReturnRows <== ".to_string() + decoded_num.to_string().as_str());
                    }
                    return result;
                } else {
                    //exec
                    let exec_result = conn_opt.unwrap().prep_exec(sql, {});
                    if exec_result.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                    }
                    let result = exec_result.unwrap();
                    let r = serde_json::from_value(json!(result.affected_rows()));
                    if r.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + r.err().unwrap().to_string().as_str());
                    }
                    if self.enable_log {
                        let affected_rows = result.affected_rows();
                        info!("{}", "[rbatis] RowsAffected <== ".to_string() + affected_rows.to_string().as_str());
                    }
                    return Result::Ok(r.unwrap());
                }
            }
            "postgres" => {
                let conn_opt = self.conn_pool.get_postage_conn("".to_string(), conf)?;
                if is_select {
                    //select
                    let exec_result = conn_opt.unwrap().query(sql, &[]);
                    if exec_result.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                    }
                    let (result, decoded_num) = exec_result.unwrap().decode();
                    if self.enable_log {
                        info!("{}", "[rbatis] ReturnRows <== ".to_string() + decoded_num.to_string().as_str());
                    }
                    return result;
                } else {
                    //exec
                    let exec_result = conn_opt.unwrap().execute(sql, &[]);
                    if exec_result.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                    }
                    let num = 0.0;
                    let mut num_opt = Number::from_f64(exec_result.unwrap() as f64);
                    if num_opt.is_none() {
                        num_opt = Number::from_f64(num);
                    }
                    let r = serde_json::from_value(serde_json::Value::Number(num_opt.unwrap()));
                    if r.is_err() {
                        return Result::Err("[rbatis] exec fail:".to_string() + r.err().unwrap().to_string().as_str());
                    }
                    return Result::Ok(r.unwrap());
                }
            }
            _ => {
                if self.enable_log {
                    error!("{}", "[rbatis] unsupport database type:".to_string() + db_type);
                }
                return Result::Err("[rbatis] unsupport database type:".to_string() + db_type);
            }
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
    pub fn eval<T>(&mut self, mapper_name: &str, id: &str, env: &mut Value) -> Result<T, String> where T: de::DeserializeOwned {
        let mapper_opt = self.mapper_map.get(&mapper_name.to_string());
        if mapper_opt.is_none() {
            return Result::Err("[rbatis] find mapper fail,name:'".to_string() + mapper_name + "'");
        }
        let node = mapper_opt.unwrap().get(id);
        if node.is_none() {
            return Result::Err("[rbatis] find method fail,name:'".to_string() + mapper_name + id + "'");
        }

        let mapper_func = node.unwrap();
        let sql_string = mapper_func.eval(env, &mut self.holder)?;
        let sql = sql_string.as_str();

        let mut db = self.get_conf(id);
        match &mapper_func {
            NodeType::NSelectNode(_) => {
                return self.eval_sql_raw(sql, true, db.as_str());
            }
            _ => {
                return self.eval_sql_raw(sql, false, db.as_str());
            }
        }
    }

    pub fn get_conf(&self, key: &str) -> String {
        let mut db = "".to_string();
        let conf = self.router_configs.get(key).unwrap_or(&db);
        return conf.clone();
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