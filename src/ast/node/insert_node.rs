use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;

use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::error::RbatisError;

#[derive(Clone,Debug)]
pub struct InsertNode {
    pub id: String,
    pub childs: Vec<NodeType>,
}

impl RbatisAST for InsertNode {
    fn eval(&self, env: &mut Value, engine: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, RbatisError> {
        return do_child_nodes(&self.childs, env,engine,arg_array);
    }
}

impl SqlNodePrint for InsertNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<insert ";
        result = result + "id=\"" + self.id.as_str() + "\"";
        result = result + ">";
        result = result + print_child(self.childs.as_ref(), deep + 1).as_str();
        result = result + create_deep(deep).as_str() + "</insert>";
        return result;
    }
}
