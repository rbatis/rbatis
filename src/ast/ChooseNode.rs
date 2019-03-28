use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::Node;
use serde_json::Value;

pub struct ChooseNode {
    pub whenNodes: Vec<NodeType>,
    pub otherwiseNode: Rc<NodeType>,
}
impl Node for ChooseNode{
    fn eval(&self, env: &Value) -> String {
        unimplemented!()
    }
}
