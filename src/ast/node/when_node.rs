use std::borrow::BorrowMut;

use serde_json::{json, Value};

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::do_child_nodes;
use crate::ast::node::node_type::NodeType;
use crate::core::convert::StmtConvert;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct WhenNode {
    pub childs: Vec<NodeType>,
    pub test: String,

}

impl WhenNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["when ".len()..].trim();
        return Ok(WhenNode {
            childs,
            test: express.to_string(),
        });
    }
}


impl RbatisAST for WhenNode {
    fn name() -> &'static str {
        "when"
    }
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let result = engine.eval(self.test.as_str(), env)?;
        if !result.is_boolean() {
            return Result::Err(crate::core::Error::from("[rbatis] test:'".to_owned() + self.test.as_str() + "' is not return bool!"));
        }
        if result.as_bool().unwrap() {
            return do_child_nodes(convert, &self.childs, env, engine, arg_array);
        }
        return Result::Ok("".to_string());
    }
}