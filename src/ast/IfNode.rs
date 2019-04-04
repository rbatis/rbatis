use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
    pub holder: Box<NodeConfigHolder>,
}

impl SqlNode for IfNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        unimplemented!()
    }
}