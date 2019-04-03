use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::OtherwiseNode::OtherwiseNode;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct ChooseNode<'a> {
    pub whenNodes: Option<Vec<NodeType<'a>>>,
    pub otherwiseNode: Option<Box<NodeType<'a>>>,
}

impl<'a> SqlNode for ChooseNode<'a> {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        let whenIsNone = self.whenNodes.is_none();
        if whenIsNone == false {
            for mut item in self.whenNodes.clone().unwrap() {
                let s = item.eval(env);
                if s.is_ok() {
                    return s;
                }
            }
        }
        if self.otherwiseNode.is_none() == false {
            return self.otherwiseNode.clone().unwrap().deref_mut().eval(env);
        }
        return Result::Ok("".to_string());
    }
}
