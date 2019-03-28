use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl Node for OtherwiseNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}
