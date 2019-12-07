use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::node_config_holder::NodeConfigHolder;

#[derive(Clone)]
pub struct InsertNode {
    pub id:String,
    pub childs: Vec<NodeType>,
}

impl SqlNode for InsertNode{
    fn eval(&mut self, env: &mut Value,holder:&mut NodeConfigHolder) -> Result<String, String> {

        return do_child_nodes(&mut self.childs, env, holder);
    }

    fn print(&self,deep:i32) -> String {
        let mut result=create_deep(deep)+"<insert ";
        result=result+"id=\""+self.id.as_str()+"\"";
        result=result+">";
        result=result+print_child(self.childs.as_ref(),deep+1).as_str();
        result=result+create_deep(deep).as_str()+"</insert>";
        return result;
    }
}
