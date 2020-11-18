use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::do_child_nodes;
use crate::ast::node::node_type::NodeType;
use crate::core::convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct OtherwiseNode {
    pub childs: Vec<NodeType>,
}

impl OtherwiseNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["otherwise".len()..].trim();
        return Ok(OtherwiseNode {
            childs,
        });
    }
}

impl RbatisAST for OtherwiseNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        return do_child_nodes(convert, &self.childs, env, engine, arg_array);
    }
}
