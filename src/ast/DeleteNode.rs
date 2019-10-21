use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, print_child, create_deep};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct DeleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for DeleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<delete ";
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=print_child(result,self.childs.as_ref(),deep+1);
        result=result+create_deep(deep).as_str()+"</delete>";
        return result;
    }
}