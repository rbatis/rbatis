use serde_json::{json, Value};

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::core::convert::StmtConvert;
use crate::interpreter::expr::runtime::ExprRuntime;

#[derive(Clone, Debug)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SetNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let trim_x = express["set".len()..].trim();
        return Ok(SetNode {
            childs
        });
    }
}

impl RbatisAST for SetNode {
    fn name() -> &'static str {
        "set"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array);
    }
}

