use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct IncludeNode<'a> {
    pub childs: Vec<NodeType<'a>>,
}

impl <'a> SqlNode for IncludeNode<'a>{
    fn eval(&mut self, env: &Value) -> String {
        unimplemented!()
    }
}