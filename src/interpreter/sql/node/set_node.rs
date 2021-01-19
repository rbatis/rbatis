use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;
use rexpr::runtime::RExprRuntime;

#[derive(Clone, Debug)]
pub struct SetNode {
    pub childs: Vec<NodeType>,
}

impl SetNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
        let trim_x = express[Self::name().len()..].trim();
        return Ok(SetNode { childs });
    }
}

impl RbatisAST for SetNode {
    fn name() -> &'static str {
        "set"
    }
    fn eval(
        &self,
        convert: &crate::core::db::DriverType,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql);
    }
}
