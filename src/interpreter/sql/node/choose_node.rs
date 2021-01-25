use core::borrow::BorrowMut;
use std::ops::DerefMut;

use serde_json::{json, Value};

use crate::core::convert::StmtConvert;

use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::node_type::NodeType;
use crate::interpreter::sql::node::node_type::NodeType::NString;
use crate::interpreter::sql::node::otherwise_node::OtherwiseNode;
use crate::interpreter::sql::node::string_node::StringNode;
use rexpr::runtime::RExprRuntime;
use crate::core::db::DriverType;

#[derive(Clone, Debug)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl ChooseNode {
    pub fn from(
        source: &str,
        express: &str,
        childs: Vec<NodeType>,
    ) -> Result<Self, crate::core::Error> {
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

    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        if self.when_nodes.is_none() == false {
            let mut when_index = 0;
            for item in self.when_nodes.as_ref().unwrap() {
                let v = item.eval(convert, env, engine, arg_array, arg_sql)?;
                if v.as_bool().unwrap_or(false) == true {
                    return Result::Ok(serde_json::Value::Null);
                }
                when_index += 1;
            }
        }
        match &self.otherwise_node {
            Some(other) => {
                return other.eval(convert, env, engine, arg_array, arg_sql);
            }
            _ => {}
        }
        return Result::Ok(serde_json::Value::Null);
    }
}

#[test]
pub fn test_choose_node() {
    let mut engine = RExprRuntime::new();
    let mut john = json!({
        "arg": 2,
    });
    let s_node = NString(StringNode::new(&engine,"dsaf#{arg+1}").unwrap());

    let c = ChooseNode {
        when_nodes: Option::Some(vec![s_node]),
        otherwise_node: None,
    };
    let mut arg_array = vec![];

    let mut r = String::new();
    c.eval(
        &DriverType::Mysql,
        &mut john,
        &mut engine,
        &mut arg_array,
        &mut r,
    );
    println!("{}", r);
}
