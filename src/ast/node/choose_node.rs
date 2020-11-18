use core::borrow::BorrowMut;
use std::ops::DerefMut;

use serde_json::{json, Value};

use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;

use crate::ast::ast::RbatisAST;
use crate::ast::node::node_type::NodeType;
use crate::ast::node::node_type::NodeType::NString;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::string_node::StringNode;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone, Debug)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl RbatisAST for ChooseNode {
    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &RbatisEngine, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
        if self.when_nodes.is_none() == false {
            for item in self.when_nodes.as_ref().unwrap() {
                let s = item.eval(convert, env, engine, arg_array);
                if s.is_ok() {
                    return s;
                }
            }
        }
        if self.otherwise_node.is_none() == false {
            return self.otherwise_node.as_ref().unwrap().eval(convert, env, engine, arg_array);
        }
        return Result::Ok("".to_string());
    }
}


#[test]
pub fn test_choose_node() {
    let mut engine = RbatisEngine::new();
    let mut john = json!({
        "arg": 2,
    });
    let s_node = NString(StringNode::new("dsaf#{arg+1}"));

    let c = ChooseNode {
        when_nodes: Option::Some(vec![s_node]),
        otherwise_node: None,
    };
    let mut arg_array = vec![];


    let r = c.eval(&DriverType::Mysql, &mut john, &mut engine, &mut arg_array);
    println!("{}", r.unwrap());
}