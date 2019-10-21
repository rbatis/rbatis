use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct InsertTempleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertTempleteNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<insertTemplete ";
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";

        result=print_child(result,self.childs.as_ref(),deep+1);
        result=result+create_deep(deep).as_str()+"</insertTemplete>";
        return result;
    }
}
