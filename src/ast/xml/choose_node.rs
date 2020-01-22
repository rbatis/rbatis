use core::borrow::BorrowMut;
use std::ops::DerefMut;
use std::rc::Rc;

use serde_json::{json, Value};

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node::{create_deep, print_child, SqlNodePrint};
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::node_type::NodeType::NString;
use crate::ast::xml::otherwise_node::OtherwiseNode;
use crate::ast::xml::string_node::StringNode;
use crate::engine::runtime::RbatisEngine;

#[derive(Clone,Debug)]
pub struct ChooseNode {
    pub when_nodes: Option<Vec<NodeType>>,
    pub otherwise_node: Option<Box<NodeType>>,
}

impl Ast for ChooseNode {
    fn eval(&self, env: &mut Value, holder: &mut ConfigHolder,arg_array:&mut Vec<Value>) -> Result<String, String> {
        if self.when_nodes.is_none() == false {
            for item in self.when_nodes.clone().unwrap() {
                let s = item.eval(env,holder,arg_array);
                if s.is_ok() {
                    return s;
                }
            }
        }
        if self.otherwise_node.is_none() == false {
            return self.otherwise_node.clone().unwrap().deref_mut().eval(env,holder,arg_array);
        }
        return Result::Ok("".to_string());
    }
}

impl SqlNodePrint for ChooseNode {
    fn print(&self, deep: i32) -> String {
        let mut result = create_deep(deep) + "<choose>";
        result = result + print_child(self.when_nodes.as_ref().unwrap(), deep + 1).as_str();
        result = result + self.otherwise_node.as_ref().unwrap().print(deep).as_str();
        result = result + create_deep(deep).as_str() + "</choose>";
        return result;
    }
}


#[test]
pub fn test_choose_node() {
    let mut holder = ConfigHolder::new();
    let mut john = json!({
        "arg": 2,
    });
    let engine = RbatisEngine::new();

    let s_node = NString(StringNode::new("dsaf#{arg+1}"));

    let c = ChooseNode {
        when_nodes: Option::Some(vec![s_node]),
        otherwise_node: None,
    };
    let mut arg_array=vec![];


    let r = c.eval(&mut john,&mut holder, &mut arg_array);
    println!("{}", r.unwrap());
}