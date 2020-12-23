use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::interpreter::expr::runtime::ExprRuntime;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node::do_child_nodes;
use crate::interpreter::sql::node::node_type::NodeType;

#[derive(Clone, Debug)]
pub struct WhereNode {
    pub childs: Vec<NodeType>,
}

impl WhereNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express[Self::name().len()..].trim();
        return Ok(WhereNode {
            childs
        });
    }
}

impl RbatisAST for WhereNode {
    fn name() -> &'static str {
        "where"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let result = do_child_nodes(convert, &self.childs, env, engine, arg_array)?;
        let s = result.trim().to_string();
        if s.is_empty() {
            return Result::Ok(" ".to_string());
        } else {
            let mut result = s.as_str().trim();
            result = result.trim_start_matches("and");
            result = result.trim_start_matches("AND");
            result = result.trim_start_matches("And");
            result = result.trim_start_matches("or");
            result = result.trim_start_matches("Or");
            result = result.trim_start_matches("OR");
            return Result::Ok(" WHERE ".to_string() + result);
        }
    }
}