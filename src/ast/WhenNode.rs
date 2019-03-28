use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl Node for WhenNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}