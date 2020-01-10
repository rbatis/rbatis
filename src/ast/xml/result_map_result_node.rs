use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, do_child_nodes, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapResultNode {
    pub column: String,
    pub property: String,
    pub lang_type: String,

    pub version_enable: String,
    pub logic_enable: String,
    pub logic_undelete: String,
    pub logic_deleted: String,
}


impl Ast for ResultMapResultNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ResultMapResultNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<result ";
        result = result + " column=\"" + self.column.as_str() + "\"";
        result = result + " property=\"" + self.property.as_str() + "\"";
        result = result + " lang_type=\"" + self.lang_type.as_str() + "\"";
        result = result + "></result>";
        return result;
    }
}