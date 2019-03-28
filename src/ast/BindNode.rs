use crate::ast::Node::Node;
use serde_json::Value;

pub struct BindNode {
    pub name: String,
    pub value: String,
}

impl Node for BindNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}
