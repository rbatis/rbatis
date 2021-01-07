use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use rexpr::runtime::RExprRuntime;
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
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RExprRuntime, arg_array: &mut Vec<Value>, arg_sql: &mut String) -> Result<serde_json::Value, crate::core::Error> {
        let mut child_sql = String::new();
        do_child_nodes(convert, &self.childs, env, engine, arg_array, &mut child_sql)?;
        let mut result = child_sql.trim();
        if result.is_empty() {
            return Result::Ok(serde_json::Value::Null);
        } else {
            result = result.trim_start_matches("and")
                .trim_start_matches("AND")
                .trim_start_matches("And")
                .trim_start_matches("or")
                .trim_start_matches("Or")
                .trim_start_matches("OR");
            arg_sql.push_str(" WHERE ");
            arg_sql.push_str(result);
            return Result::Ok(serde_json::Value::Null);
        }
    }
}