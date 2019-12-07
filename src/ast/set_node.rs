use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, DoChildNodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::node_config_holder::NodeConfigHolder;

#[derive(Clone)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SqlNode for SetNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<set>";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</set>";
        return result;
    }

}