use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes, print_child};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct SelectNode {
    pub id:String,
    pub resultMap:String,
    pub childs: Vec<NodeType>,
}


impl SqlNode for SelectNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self) -> String {
        let mut result="\n<select ".to_string();
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=print_child(result,self.childs.as_ref());
        result=result+" \n</select>";
        return result;
    }
}