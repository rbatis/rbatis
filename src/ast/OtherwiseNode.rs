use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl SqlNode for OtherwiseNode {
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String,String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }
    fn print(&self) -> String {
        let mut result="\n<otherwise>".to_string();
        for x in &self.childs {
            result=result+x.print().as_str();
        }
        result+=" </otherwise>";
        return result;
    }
}
