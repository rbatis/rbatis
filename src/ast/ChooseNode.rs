use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::SqlNode;
use serde_json::Value;

pub struct ChooseNode<'a> {
    pub whenNodes: Option<Vec<NodeType<'a>>>,
    pub otherwiseNode: Option<Rc<NodeType<'a>>>,
}

impl<'a> SqlNode for ChooseNode<'a> {
    fn eval(&mut self, env: &mut Value) -> String {
        unimplemented!()
//        let whenIsNone = self.whenNodes.is_none();
//        let otherIsNone = self.otherwiseNode.is_none();
//        if whenIsNone || otherIsNone {
//            return String::new();
//        }
//        for mut item in self.whenNodes.unwrap() {
//            let s = item.eval(env);
//
//        }
//
//        return String::new();
    }
}
