use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::node::{SqlNode, print_child, create_deep, do_child_nodes, SqlNodePrint};
use serde_json::{Value,json};
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct DeleteNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for DeleteNode{
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        return do_child_nodes(&self.childs, env, holder);
    }
}

impl SqlNodePrint for DeleteNode{
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<delete ";
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</delete>";
        return result;
    }
}