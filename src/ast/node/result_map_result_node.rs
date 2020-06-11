use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;

use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::error::RbatisError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapResultNode {
    pub column: String,
    pub lang_type: String,

    pub version_enable: String,
    pub logic_enable: String,
    pub logic_undelete: String,
    pub logic_deleted: String,
}


impl RbatisAST for ResultMapResultNode {
    fn eval(&self, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, RbatisError> {
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ResultMapResultNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<result ";
        result = result + " column=\"" + self.column.as_str() + "\"";
        result = result + " lang_type=\"" + self.lang_type.as_str() + "\"";
        result = result + "></result>";
        return result;
    }
}