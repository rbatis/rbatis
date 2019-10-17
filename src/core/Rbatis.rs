use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::Node::{SqlNode, LoopDecodeXml};
use crate::ast::BindNode::BindNode;
use crate::ast::StringNode::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;
use crate::ast::NodeType::NodeType;

pub struct  Rbatis{

}


impl Rbatis{

    pub fn build(&self,xml_content:String) -> Vec<NodeType>{
        //TODO load xml_content string,create ast
        let holder=NodeConfigHolder::new();
        let nodes= load_xml(xml_content);
        let result= LoopDecodeXml(nodes,holder.clone());
        return result;
    }
}