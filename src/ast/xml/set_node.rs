use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;

#[derive(Clone)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl Ast for SetNode {
    fn eval(&self, env: &mut Value, arg_array:&mut Vec<Value>,holder: &mut ConfigHolder) -> Result<String, String> {
        return do_child_nodes(&self.childs, env, arg_array,holder);
    }
}

impl SqlNodePrint for SetNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<set>";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</set>";
        return result;
    }
}