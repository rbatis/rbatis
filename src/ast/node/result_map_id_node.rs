use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::Ast;

use crate::ast::node::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::node::node_type::NodeType;
use crate::engine::runtime::RbatisEngine;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapIdNode {
    pub column: String,
    pub property: String,
    pub lang_type: String,
}


impl Ast for ResultMapIdNode {
    fn eval(&self, env: &mut Value, holder: &mut RbatisEngine,arg_array:&mut Vec<Value>) -> Result<String, String> {
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ResultMapIdNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<id ";
        result = result + " column=\"" + self.column.as_str() + "\"";
        result = result + " property=\"" + self.property.as_str() + "\"";
        result = result + " lang_type=\"" + self.lang_type.as_str() + "\"";
        result = result + "></id>";
        return result;
    }
}