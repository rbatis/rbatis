use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;

#[derive(Clone)]
pub struct UpdateNode {
    pub id: String,
    pub childs: Vec<NodeType>,
}


impl Ast for UpdateNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        return do_child_nodes(&self.childs, env,holder,arg_array);
    }
}

impl SqlNodePrint for UpdateNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<update ";
        result = result + "id=\"" + self.id.as_str() + "\"";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</update>";
        return result;
    }
}