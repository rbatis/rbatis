use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

#[derive(Clone)]
pub struct OtherwiseNode<'a> {
    pub childs: Vec<NodeType<'a>>,
}

impl<'a> SqlNode for OtherwiseNode<'a> {
    fn eval(&mut self, env: &mut Value) -> Result<String,String> {
        unimplemented!()
    }
}
