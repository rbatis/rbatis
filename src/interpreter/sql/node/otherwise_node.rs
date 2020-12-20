use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl OtherwiseNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["otherwise".len()..].trim();
        return Ok(OtherwiseNode {
            childs,
        });
    }
}

impl RbatisAST for OtherwiseNode {
    fn name() -> &'static str {
        "otherwise"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array);
    }
}
