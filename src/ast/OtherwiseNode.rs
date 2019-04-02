use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct OtherwiseNode<'a> {
    pub childs: Vec<NodeType<'a>>,
}

impl<'a> SqlNode for OtherwiseNode<'a> {
    fn eval(&mut self, env: &Value) -> String {
        unimplemented!()
    }
}
