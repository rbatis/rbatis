use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::SqlNode;
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::OtherwiseNode::OtherwiseNode;
use std::ops::DerefMut;

#[derive(Clone)]
pub struct ChooseNode {
    pub whenNodes: Option<Vec<NodeType>>,
    pub otherwiseNode: Option<Box<NodeType>>,
}

impl SqlNode for ChooseNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        if self.whenNodes.is_none() == false {
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
