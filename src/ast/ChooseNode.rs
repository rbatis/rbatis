use crate::ast::NodeType::NodeType;
use std::rc::Rc;
use crate::ast::Node::{SqlNode, print_child};
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::OtherwiseNode::OtherwiseNode;
use std::ops::DerefMut;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct ChooseNode {
    pub whenNodes: Option<Vec<NodeType>>,
    pub otherwiseNode: Option<Box<NodeType>>,
}

impl SqlNode for ChooseNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        if self.whenNodes.is_none() == false {
            for mut item in self.whenNodes.clone().unwrap() {
                let s = item.eval(env,holder);
                if s.is_ok() {
                    return s;
                }
            }
        }
        if self.otherwiseNode.is_none() == false {
            return self.otherwiseNode.clone().unwrap().deref_mut().eval(env,holder);
        }
        return Result::Ok("".to_string());
    }

    fn print(&self) -> String {
        let mut result= "\n<choose>".to_string();
        result=print_child(result,self.whenNodes.as_ref().unwrap());
        result=result+self.otherwiseNode.as_ref().unwrap().print().as_str();
        result=result+" \n</choose>";
        return result;
    }
}
