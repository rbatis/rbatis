use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, create_deep, print_child, SqlNodePrint};
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct IncludeNode {
    pub refid: String,
    pub childs: Vec<NodeType>,
}

impl  SqlNode for IncludeNode{

    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String,String> {
        return do_child_nodes(&self.childs, env, holder);
    }
}

impl SqlNodePrint for IncludeNode{
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<include "+"refid=\""+ self.refid.as_str()+"\"" +" >";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+ create_deep(deep).as_str()+"</include>";
        return result;
    }
}