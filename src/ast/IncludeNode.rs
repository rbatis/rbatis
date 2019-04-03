use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct IncludeNode {
    pub childs: Vec<NodeType>,
}

impl  SqlNode for IncludeNode{
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}