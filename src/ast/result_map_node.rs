
use crate::ast::node_type::NodeType;
use std::rc::Rc;
use crate::ast::node::{SqlNode, print_child, create_deep};
use serde_json::Value;
use core::borrow::BorrowMut;
use crate::ast::otherwise_node::OtherwiseNode;
use std::ops::DerefMut;
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct ResultMapNode {
    pub id:String,
    pub id_node: Option<Box<NodeType>>,
    pub results: Option<Vec<NodeType>>,
}

impl SqlNode for ResultMapNode {
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        return Result::Ok("".to_string());
    }

    fn print(&self,deep:i32) -> String {
        let mut result= create_deep(deep)+"<result_map id=\""+self.id.as_str()+"\">";
        if (self.id_node.is_some()){
            result=result+self.id_node.as_ref().unwrap().print(deep).as_str();
        }
        if self.results.is_some(){
            result=result+print_child(self.results.as_ref().unwrap(), deep+1).as_str();
        }
        result=result+create_deep(deep).as_str()+"</result_map>";
        return result;
    }
}
