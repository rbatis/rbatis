use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct WhenNode<'a> {
    pub childs: Vec<NodeType<'a>>,
    pub test: String,
}

impl <'a> SqlNode for WhenNode<'a>{
    fn eval(&mut self, env: &Value) -> String {
        unimplemented!()
    }
}