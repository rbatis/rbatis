use crate::ast::node_type::NodeType;
use std::rc::Rc;
use crate::ast::node::{SqlNode, print_child, create_deep};
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::otherwise_node::OtherwiseNode;
use std::ops::DerefMut;
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl SqlNode for ChooseNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        if self.when_nodes.is_none() == false {
            for item in self.when_nodes.clone().unwrap() {
                let s = item.eval(env,holder);
                if s.is_ok() {
                    return s;
                }
            }
        }
        if self.otherwise_node.is_none() == false {
            return self.otherwise_node.clone().unwrap().deref_mut().eval(env, holder);
        }
        return Result::Ok("".to_string());
    }

    fn print(&self,deep:i32) -> String {
        let mut result= create_deep(deep)+"<choose>";
        result=result+print_child(self.when_nodes.as_ref().unwrap(), deep+1).as_str();
        result=result+self.otherwise_node.as_ref().unwrap().print(deep).as_str();
        result=result+create_deep(deep).as_str()+"</choose>";
        return result;
    }
}
