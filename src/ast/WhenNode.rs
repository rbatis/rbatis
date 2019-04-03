use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl  SqlNode for WhenNode{
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}