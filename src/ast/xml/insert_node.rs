use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::node::{SqlNode, do_child_nodes, print_child, create_deep, SqlNodePrint};
use serde_json::{Value,json};
use crate::ast::config_holder::ConfigHolder;

#[derive(Clone)]
pub struct InsertNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertNode{
    fn eval(&self, env: &mut Value, holder:&mut ConfigHolder) -> Result<String, String> {
        return do_child_nodes(&self.childs, env, holder);
    }
}

impl SqlNodePrint for InsertNode{
    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<insert ";
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</insert>";
        return result;
    }
}
