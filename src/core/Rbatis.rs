use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::Node::{SqlNode, LoopDecodeXml};
use crate::ast::BindNode::BindNode;
use crate::ast::StringNode::StringNode;
use crate::utils::xml_loader::load_xml;
use std::rc::Rc;

pub struct  Rbatis{

}


impl Rbatis{

    pub fn build(&self,xml_content:String) -> impl SqlNode{
        //TODO load xml_content string,create ast
        let holder=NodeConfigHolder::new();
        let nodes= load_xml(xml_content);
        let result= LoopDecodeXml(nodes,holder.clone());
        let mut bindNode =StringNode::new("----------Str---------",holder);
        return bindNode;
    }
}