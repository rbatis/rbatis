use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::{SqlNode, loop_decode_xml};
use crate::ast::bind_node::BindNode;
use crate::ast::string_node::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;
use crate::ast::node_type::NodeType;
use serde_json::{Value, Number};
use std::collections::HashMap;
use crate::core::db_config::DBConfig;
use rbatis_macro::RbatisMacro;
use serde::de;
use std::str::FromStr;
use crate::core::conn_pool::ConnPool;
use std::sync::Mutex;
use crate::utils::driver_util;
use mysql::Conn;
use crate::decode::decoder::Decoder;
use crate::core::node_type_map_factory::create_node_type_map;
use std::any::Any;
use std::process::exit;
use serde_json::ser::State::Rest;
use serde_json::json;

pub struct Rbatis {
    pub mapper_map: HashMap<String, HashMap<String, NodeType>>,
    //动态sql运算节点集合
    pub holder: ConfigHolder,
    //动态sql节点配置
    pub db_configs: HashMap<String, DBConfig>,
    //数据库连接配置
    pub conn_pool: ConnPool,
}

impl Rbatis {
    pub fn new() -> Rbatis {
        return Rbatis {
            mapper_map: HashMap::new(),
            holder: ConfigHolder::new(),
            db_configs: HashMap::new(),
            conn_pool: ConnPool::new(),
        };
    }

    pub fn load_xml(&mut self, key: String, content: String) {
        self.mapper_map.insert(key, create_node_type_map(content, &self.holder));
    }


    /// 设置数据库默认url，如果失败返回错误信息
    pub fn load_db_url(&mut self, name: String, url: String) -> Option<String> {
        let db_config_opt = DBConfig::new(url);
        if db_config_opt.is_ok() {
            if name.is_empty() {
                self.db_configs.insert("".to_string(), db_config_opt.unwrap());
            } else {
                self.db_configs.insert(name, db_config_opt.unwrap());
            }
            return Option::None;
        } else {
            let e = db_config_opt.err().unwrap();
            println!("{}", "[rbatis] link db fail:".to_string() + e.as_str());
            return Option::Some(e);
        }
    }
    pub fn remove_db_url(&mut self, name: String) {
        self.db_configs.remove(&name);
    }


    pub fn eval<T>(&mut self, mapper_name: String, id: &str, env: &mut Value) -> Result<T, String> where T: de::DeserializeOwned + RbatisMacro {
        let mapper_opt = self.mapper_map.get_mut(&mapper_name);
        if mapper_opt.is_none() {
            return Result::Err("[rbatis] find mapper fail,name:'".to_string() + mapper_name.to_string().as_str() + "'");
        }
        let mut node = mapper_opt.unwrap().get_mut(id);
        if node.is_none() {
            return Result::Err("[rbatis] find method fail,name:'".to_string() + mapper_name.to_string().as_str() + id + "'");
        }
        let mapper_func = node.unwrap();
        let sql = mapper_func.eval(env, &mut self.holder)?;
        println!("[rbatis] Query ==>  {}", sql);
        let conf_opt = self.db_configs.get("");
        if conf_opt.is_none() {
            return Result::Err("[rbatis] find default database url config fail！".to_string());
        }
        let conf = conf_opt.unwrap();
        let db_type = conf.db_type.as_str();
        match db_type {
            "mysql" => {
                let conn_opt = self.conn_pool.get_mysql_conn("".to_string(), conf)?;
                match mapper_func {
                    NodeType::NSelectNode(node) => {
                        //select
                        let exec_result = conn_opt.unwrap().prep_exec(sql, {});
                        if exec_result.is_err() {
                            return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                        }
                        return exec_result.unwrap().decode();
                    }
                    _ => {
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
                        return Result::Ok(r.unwrap());
                    }
                }
            }
            "postgres" => {
                let conn_opt = self.conn_pool.get_postage_conn("".to_string(), conf)?;
                //TODO conn_opt.unwrap().query 做 query 和exec
                match mapper_func {
                    NodeType::NSelectNode(node) => {
                        //select
                        let exec_result = conn_opt.unwrap().query(sql.as_str(), &[]);
                        if exec_result.is_err() {
                            return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                        }
                        return exec_result.unwrap().decode();
                    }
                    _ => {
                        //exec
                        let exec_result = conn_opt.unwrap().execute(sql.as_str(), &[]);
                        if exec_result.is_err() {
                            return Result::Err("[rbatis] exec fail:".to_string() + exec_result.err().unwrap().to_string().as_str());
                        }
                        let mut num = 0.0;
                        let mut numOpt = Number::from_f64(exec_result.unwrap() as f64);
                        if numOpt.is_none() {
                            numOpt = Number::from_f64(num);
                        }
                        let r = serde_json::from_value(serde_json::Value::Number(numOpt.unwrap()));
                        if r.is_err() {
                            return Result::Err("[rbatis] exec fail:".to_string() + r.err().unwrap().to_string().as_str());
                        }
                        return Result::Ok(r.unwrap());
                    }
                }
            }
            _ => return Result::Err("[rbatis] unsupport database type:".to_string() + db_type)
        }
    }

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
}