use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct DeleteNode {
    pub childs: Vec<NodeType>,
}

impl SqlNode for DeleteNode{
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        unimplemented!()
    }
}