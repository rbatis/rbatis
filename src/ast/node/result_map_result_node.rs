use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::RbatisSqlAST;
use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::convert::stmt_convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapResultNode {
    pub column: String,
    pub lang_type: String,

    pub version_enable: String,
    pub logic_enable: String,
    pub logic_undelete: String,
    pub logic_deleted: String,
}


impl RbatisSqlAST for ResultMapResultNode {
    fn eval(&self, convert: &impl StmtConvert, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, rbatis_core::Error> {
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