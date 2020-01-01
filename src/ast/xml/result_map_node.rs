use core::borrow::BorrowMut;
use std::ops::DerefMut;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::otherwise_node::OtherwiseNode;
use crate::ast::xml::result_map_id_node::ResultMapIdNode;
use crate::ast::xml::result_map_result_node::ResultMapResultNode;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultMapNode {
    pub id: String,
    pub id_node: Option<ResultMapIdNode>,
    pub results: Vec<ResultMapResultNode>,
}

impl Ast for ResultMapNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder) -> Result<String, String> {
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ResultMapNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<result_map id=\"" + self.id.as_str() + "\">";
        if self.id_node.is_some() {
            result = result + self.id_node.as_ref().unwrap().print(deep).as_str();
        }
        if self.results.len() > 0 {
            result = result + print_child(self.results.as_ref(), deep + 1).as_str();
        }
        result = result + create_deep(deep).as_str() + "</result_map>";
        return result;
    }
}

