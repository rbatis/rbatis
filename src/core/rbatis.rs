use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::{SqlNode, loop_decode_xml};
use crate::ast::bind_node::BindNode;
use crate::ast::string_node::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;
use crate::ast::node_type::NodeType;
use serde_json::Value;
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

pub struct Rbatis {
    node_types: HashMap<String, NodeType>,
    //动态sql运算节点集合
    holder: ConfigHolder,
    //动态sql节点配置
    db_configs: HashMap<String, DBConfig>,
    //数据库连接配置
   pub conn_pool: ConnPool,
}

impl Rbatis {
    pub fn new(xml_content: String) -> Rbatis {
        //TODO load xml_content string,create ast
        let holder = ConfigHolder::new();
        let nodes = load_xml(xml_content);
        let data = loop_decode_xml(&nodes, &holder);
        let mut m = HashMap::new();
        for x in data {
            match x.clone() {
                NodeType::NSelectNode(node) => m.insert(node.id, x),
                NodeType::NDeleteNode(node) => m.insert(node.id, x),
                NodeType::NUpdateNode(node) => m.insert(node.id, x),
                NodeType::NInsertNode(node) => m.insert(node.id, x),

                _ => m.insert("unknow".to_string(), NodeType::Null),
            };
        }
        return Rbatis {
            holder: holder,
            node_types: m,
            db_configs: HashMap::new(),
            conn_pool: ConnPool::new(),
        };
    }

    /// 设置数据库默认url，如果失败返回错误信息
    pub fn set_db_url(&mut self, name: String, url: String) -> Option<String> {
        let db_config_opt = DBConfig::new(url);
        if db_config_opt.is_ok() {
            if name.is_empty() {
                self.db_configs.insert("default".to_string(), db_config_opt.unwrap());
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


    pub fn eval<T>(&mut self, id: &str, env: &mut Value) -> Result<T, String> where T: de::DeserializeOwned + RbatisMacro {
        let mut node = self.node_types.get_mut(id);
        if node.is_none() {
            return Result::Err("[rbatis] find method fail:".to_string() + id + " is none");
        }
        let sql = node.unwrap().eval(env, &mut self.holder)?;
        let conf_opt = self.db_configs.get("default");
        if conf_opt.is_none() {
            return Result::Err("[rbatis] find database url config fail!".to_string());
        }
        let conf = conf_opt.unwrap();
        let db_type = conf.db_type.as_str();
        match db_type {
            "mysql" => {
                let conn;
                let conn_opt = self.conn_pool.mysql_map.get_mut(&"default".to_string());
                if conn_opt.is_none() {
                    let mysql_coon_opt=driver_util::get_mysql_conn(conf);
                    if mysql_coon_opt.is_err(){
                        return Result::Err("[rbatis] link mysql fail:".to_string() + mysql_coon_opt.err().unwrap().as_str());
                    }
                    self.conn_pool.mysql_map.insert("default".to_string(),mysql_coon_opt.unwrap());
                    conn = self.conn_pool.mysql_map.get_mut(&"default".to_string()).unwrap();
                }else{
                    conn = conn_opt.unwrap();
                }
                println!("sql:{}",sql);
                let exec_result = conn.prep_exec(sql, {});
                if exec_result.is_err(){
                    return Result::Err("[rbatis] exec fail:".to_string()+exec_result.err().unwrap().to_string().as_str());
                }
                return exec_result.unwrap().decode();
            }
            "postgres" => {}
            _ => return Result::Err("[rbatis] unsupport database type:".to_string() + db_type)
        }


        let vv = serde_json::from_str(r#"{"a":1}"#).unwrap();
        return Result::Ok(vv);
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        for (key, node) in &self.node_types {
            let data = node.print(0);
            let data_str = data.as_str();
            result += data_str;
            println!("\n{}", data_str);
        }
        return result;
    }
}