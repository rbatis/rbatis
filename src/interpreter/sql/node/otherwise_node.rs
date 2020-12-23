use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::Error;
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl OtherwiseNode {
    pub fn def_name() -> &'static str {
        "_"
    }

    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let source = source.trim();
        if source.starts_with(Self::def_name()) {
            return Ok(OtherwiseNode {
                childs,
            });
        } else if source.starts_with(Self::name()) {
            return Ok(OtherwiseNode {
                childs,
            });
        }
        return Err(Error::from("[rbaits] OtherwiseNode must start with '_:' or 'otherwise:'"));
    }
}

impl RbatisAST for OtherwiseNode {
    fn name() -> &'static str {
        "otherwise"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>, arg_sql: &mut String) -> Result<serde_json::Value, crate::core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array, arg_sql);
    }
}
