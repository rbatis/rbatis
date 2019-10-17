use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;

#[derive(Clone)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SqlNode for SetNode {
    fn eval(&mut self, env: &mut Value) -> Result<String, String> {
        return DoChildNodes(&mut self.childs, env);
    }
    fn print(&self) -> String {
        let mut result="<set>".to_string();
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        return result;
    }

}