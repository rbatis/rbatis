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

pub struct Rbatis {
    node_types: HashMap<String, NodeType>,
    //动态sql运算节点集合
    holder: ConfigHolder,
    //动态sql节点配置
    db_configs: HashMap<String, DBConfig>,//数据库连接配置
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
        };
    }

    /// 设置数据库默认url，如果失败返回错误信息
    pub fn set_db_url(&mut self, name: String, url: String)->Option<String>{
        let db_config_opt = DBConfig::new(url);
        if db_config_opt.is_ok() {
            self.db_configs.insert(name, db_config_opt.unwrap());
            return Option::None;
        }else{
            return Option::Some(db_config_opt.err().unwrap());
        }
    }
    pub fn remove_db_url(&mut self, name: String) {
        self.db_configs.remove(&name);
    }


    pub fn eval(&mut self, id: &str, env: &mut Value) -> Result<String, String> {
        let mut node = self.node_types.get_mut(id);
        if node.is_none() {
            return Result::Err("[rbatis] find method fail:".to_string() + id + " is none");
        }
        let sql= node.unwrap().eval(env, &mut self.holder)?;
        let conf_opt=self.db_configs.get("");
        if conf_opt.is_none(){
            return Result::Err("[rbatis] find database url config fail!".to_string());
        }
        let conf=conf_opt.unwrap();
        let db_type=conf.db_type.as_str();
        match db_type {
            "mysql" => {

            },
            "postgres" => {

            },
            _ =>   return Result::Err("[rbatis] unsupport database type:".to_string()+db_type)
        }
        return Result::Ok(sql);
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