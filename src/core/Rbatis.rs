use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::Node::{SqlNode, LoopDecodeXml};
use crate::ast::BindNode::BindNode;
use crate::ast::StringNode::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;
use crate::ast::NodeType::NodeType;
use serde_json::Value;
use std::collections::HashMap;

pub struct Rbatis {
    nodeTypes: HashMap<String,NodeType>,
    holder: NodeConfigHolder,
}

impl Rbatis {
    pub fn new(xml_content: String) -> Rbatis {
        //TODO load xml_content string,create ast
        let holder = NodeConfigHolder::new();
        let nodes = load_xml(xml_content);
        let data=LoopDecodeXml(&nodes, &holder);
        let mut m=HashMap::new();
        for x in data {
            match x.clone() {
                NodeType::NSelectNode(node) => m.insert(node.id,x),
                NodeType::NDeleteNode(node) => m.insert(node.id,x),
                NodeType::NUpdateNode(node) => m.insert(node.id,x),
                NodeType::NInsertNode(node) => m.insert(node.id,x),

                NodeType::NSelectTempleteNode(node) => m.insert(node.id,x),
                NodeType::NDeleteTempleteNode(node) => m.insert(node.id,x),
                NodeType::NUpdateTempleteNode(node) => m.insert(node.id,x),
                NodeType::NInsertTempleteNode(node) => m.insert(node.id,x),

                _ => m.insert("unknow".to_string(),NodeType::Null),
            };
        }
        return Rbatis {
            holder:holder,
            nodeTypes: m,
        };
    }

    pub fn eval(&mut self,id:&str,env: &mut Value) -> Result<String, String>{
        let mut node=self.nodeTypes.get_mut(id);
        if node.is_none(){
            return Result::Err("node:".to_string()+id+" is none");
        }
        return node.unwrap().eval(env,&mut self.holder)
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        for (key,node ) in &self.nodeTypes {
            let data = node.print(0);
            let data_str = data.as_str();
            result += data_str;
            println!("\n{}", data_str);
        }
        return result;
    }
}