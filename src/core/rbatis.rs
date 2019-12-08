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
    node_types: HashMap<String,NodeType>,
    holder: ConfigHolder,
    db_config: DBConfig,
}

impl Rbatis {
    pub fn new(xml_content: String,link:String) -> Rbatis {
        //TODO load xml_content string,create ast
        let holder = ConfigHolder::new();
        let nodes = load_xml(xml_content);
        let data= loop_decode_xml(&nodes, &holder);
        let mut m=HashMap::new();
        for x in data {
            match x.clone() {
                NodeType::NSelectNode(node) => m.insert(node.id,x),
                NodeType::NDeleteNode(node) => m.insert(node.id,x),
                NodeType::NUpdateNode(node) => m.insert(node.id,x),
                NodeType::NInsertNode(node) => m.insert(node.id,x),

                _ => m.insert("unknow".to_string(),NodeType::Null),
            };
        }
        return Rbatis {
            holder:holder,
            node_types: m,
            db_config:DBConfig::new(link)
        };
    }

    pub fn eval(&mut self,id:&str,env: &mut Value) -> Result<String, String>{
        let mut node=self.node_types.get_mut(id);
        if node.is_none(){
            return Result::Err("node:".to_string()+id+" is none");
        }
        return node.unwrap().eval(env,&mut self.holder)
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        for (key,node ) in &self.node_types {
            let data = node.print(0);
            let data_str = data.as_str();
            result += data_str;
            println!("\n{}", data_str);
        }
        return result;
    }
}