use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Node for IfNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}