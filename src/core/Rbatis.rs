use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::Node::{SqlNode, LoopDecodeXml};
use crate::ast::BindNode::BindNode;
use crate::ast::StringNode::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;
use crate::ast::NodeType::NodeType;
use serde_json::Value;

pub struct Rbatis {
    nodeTypes: Vec<NodeType>,
}

impl Rbatis {
    pub fn new(xml_content: String) -> Rbatis {
        //TODO load xml_content string,create ast
        let holder = NodeConfigHolder::new();
        let nodes = load_xml(xml_content);
        return Rbatis {
            nodeTypes: LoopDecodeXml(nodes, holder.clone()),
        };
    }

    pub fn Get(&mut self,id:&str)->NodeType{
        let node:NodeType;
        for x in &mut self.nodeTypes {
            match x {
                NodeType::NSelectNode(n) => return x.clone(),
                NodeType::NUpdateNode(n) => return x.clone(),
                NodeType::NInsertNode(n) => return x.clone(),
                NodeType::NDeleteNode(n) => return x.clone(),

                NodeType::NSelectTempleteNode(n) => return x.clone(),
                NodeType::NUpdateTempleteNode(n) => return x.clone(),
                NodeType::NInsertTempleteNode(n) => return x.clone(),
                NodeType::NDeleteTempleteNode(n) => return x.clone(),
                _ => {}
            }
        }
        return NodeType::Null;
    }

    pub fn print(&self) -> String {
        let mut result = String::new();
        for x in &self.nodeTypes {
            let data = x.print();
            let data_str = data.as_str();
            result += data_str;
            println!("\n{:?}", data_str);
        }
        return result;
    }
}