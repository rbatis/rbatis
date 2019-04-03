use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct TrimNode {
    pub childs: Vec<NodeType>,
    pub prefix: String,
    pub suffix: String,
    pub suffixOverrides: String,
    pub prefixOverrides: String,
}

impl SqlNode for TrimNode {
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}