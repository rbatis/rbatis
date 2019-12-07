use crate::ast::node_type::NodeType;
use std::rc::Rc;
use crate::ast::node::{SqlNode, print_child, create_deep};
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::otherwise_node::OtherwiseNode;
use std::ops::DerefMut;
use crate::ast::node_config_holder::NodeConfigHolder;

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

    fn print(&self,deep:i32) -> String {
        let mut result= create_deep(deep)+"<choose>";
        result=result+print_child(self.whenNodes.as_ref().unwrap(),deep+1).as_str();
        result=result+self.otherwiseNode.as_ref().unwrap().print(deep).as_str();
        result=result+create_deep(deep).as_str()+"</choose>";
        return result;
    }
}
