use crate::ast::node_type::NodeType;
use crate::ast::node::{SqlNode, do_child_nodes, print_child, create_deep};
use serde_json::Value;
use crate::ast::config_holder::ConfigHolder;
use std::borrow::BorrowMut;

#[derive(Clone)]
pub struct ResultMapResultNode {
    pub column: String,
    pub property: String,
    pub lang_type: String,
}


impl SqlNode for ResultMapResultNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        return Result::Ok("".to_string());
    }

    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<result ";
        result = result + " column=\"" + self.column.as_str() + "\"";
        result = result + " property=\"" + self.property.as_str() + "\"";
        result = result + " lang_type=\"" + self.lang_type.as_str() + "\"";
        result = result + "></result>";
        return result;
    }
}