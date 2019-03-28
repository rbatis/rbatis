use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct IncludeNode {
    pub childs: Vec<NodeType>,
}

impl Node for IncludeNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}