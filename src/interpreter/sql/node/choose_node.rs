use core::borrow::BorrowMut;
use std::ops::DerefMut;

use serde_json::{json, Value};

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::node_type::NodeType::NString;
use crate::interpreter::sql::node::otherwise_node::OtherwiseNode;
use crate::interpreter::sql::node::string_node::StringNode;
use crate::core::convert::StmtConvert;
use crate::core::db::DriverType;
use crate::interpreter::expr::runtime::ExprRuntime;

#[derive(Clone, Debug)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl ChooseNode {
    pub fn from(source: &str, express: &str, childs: Vec<NodeType>) -> Result<Self, crate::core::Error> {
        let express = express["choose".len()..].trim();
        let mut node = ChooseNode {
            when_nodes: None,
            otherwise_node: None,
        };
        for x in childs {
            match x {
                NodeType::NWhen(_) => {
                    if node.when_nodes.is_none() {
                        node.when_nodes = Some(vec![]);
                    }
                    node.when_nodes.as_mut().unwrap().push(x);
                }
                NodeType::NOtherwise(_) => {
                    node.otherwise_node = Some(Box::new(x));
                }
                _ => {
                    return Err(crate::core::Error::from("[rbatis] parser node fail,choose node' child must be when and otherwise nodes!".to_string()));
                }
            }
        }
        return Ok(node);
    }
}

impl RbatisAST for ChooseNode {
    fn name() -> &'static str {
        "choose"
    }

    fn eval(&self, convert: &crate::core::db::DriverType, env: &mut Value, engine: &ExprRuntime, arg_array: &mut Vec<Value>) -> Result<String, crate::core::Error> {
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
    let mut engine = ExprRuntime::new();
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