use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, print_child};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct UpdateTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for UpdateTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        unimplemented!()
    }

    fn print(&self) -> String {
        let mut result="\n<updateTemplete ".to_string();
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=print_child(result,self.childs.as_ref());
        result+=" \n</updateTemplete>";
        return result;
    }
}