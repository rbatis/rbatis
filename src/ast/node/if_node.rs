use serde_json::{json, Value};
use serde_json::ser::State::Rest;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node::do_child_nodes;
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;
use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct IfNode {
    pub childs: Vec<NodeType>,
    pub test: String,
}

impl IfNode {
    pub fn from(express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["if".len()..].trim();
        return Ok(IfNode {
            childs: childs,
            test: express.to_string(),
        });
    }
}

impl RbatisAST for IfNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        let result = engine.eval(self.test.as_str(), env)?;
        if !result.is_boolean() {
            return Result::Err(crate::core::Error::from("[rbatis] express:'".to_owned() + self.test.as_str() + "' is not return bool value!"));
        }
        if result.as_bool().unwrap() {
            return do_child_nodes(convert, &self.childs, env, engine, arg_array);
        }
        return Result::Ok("".to_string());
    }
}

#[test]
pub fn test_if_node() {
    let node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes"))],
        test: "arg == 1".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut engine = RbatisEngine::new();
    let mut arg_array = vec![];

    println!("{}", node.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array).unwrap());
}