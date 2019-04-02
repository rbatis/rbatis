use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct ChooseNode<'a> {
    pub whenNodes: Vec<NodeType<'a>>,
    pub otherwiseNode: Rc<NodeType<'a>>,
}
impl <'a> SqlNode for ChooseNode<'a>{
    fn eval(&mut self, env: &mut Value) -> String {
        unimplemented!()
    }
}
