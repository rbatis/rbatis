use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub t: NodeType,
    pub test: String,
}

impl SqlNode for IfNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        unimplemented!()
    }
}