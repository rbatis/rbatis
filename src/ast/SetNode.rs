use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes, print_child};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SqlNode for SetNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }
    fn print(&self) -> String {
        let mut result="\n<set>".to_string();
        result=print_child(result,self.childs.as_ref());
        result+=" \n</set>";
        return result;
    }

}