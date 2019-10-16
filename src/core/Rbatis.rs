use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::Node::SqlNode;
use crate::ast::BindNode::BindNode;
use crate::ast::StringNode::StringNode;

pub struct  Rbatis{

}


impl Rbatis{

    pub fn build(&self,xml_content:String) -> impl SqlNode{
        //TODO load xml_content string,create ast
        let holder=NodeConfigHolder::new();
        let mut bindNode =StringNode::new("----------Str---------",Box::new(holder));
        return bindNode;
    }
}