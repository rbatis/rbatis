use crate::ast::NodeType::NodeType;
use crate::ast::Node::{SqlNode, DoChildNodes, create_deep};
use serde_json::Value;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[derive(Clone)]
pub struct IncludeNode {
    pub refid: String,
    pub childs: Vec<NodeType>,
}

impl  SqlNode for IncludeNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String,String> {
        return DoChildNodes(&mut self.childs, env,holder);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<include "+"refid=\""+ self.refid.as_str()+"\"" +" >";

        for x in &self.childs {
            result=result+x.print(deep).as_str();
        }
        result=result+ create_deep(deep).as_str()+"</include>";
        return result;
    }
}