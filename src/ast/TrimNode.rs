use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct TrimNode<'a> {
    pub childs: Vec<NodeType<'a>>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

impl<'a> SqlNode for TrimNode<'a> {
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}