use serde_json::{json, Value};

use crate::ast::ast::Ast;

use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone,Debug)]
pub struct DeleteNode {
    pub id: String,
    pub childs: Vec<NodeType>,
}

impl Ast for DeleteNode {
    fn eval(&self, env: &mut Value, engine: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, String> {
        return do_child_nodes(&self.childs, env,engine,arg_array);
    }
}

impl SqlNodePrint for DeleteNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<delete ";
        result = result + "id=\"" + self.id.as_str() + "\"";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</delete>";
        return result;
    }
}