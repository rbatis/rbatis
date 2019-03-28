use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

impl Node for TrimNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}