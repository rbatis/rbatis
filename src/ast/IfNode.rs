use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct IfNode<'a> {
    pub childs: Vec<NodeType<'a>>,
    pub test: String,
}

impl <'a> SqlNode for IfNode<'a>{
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}