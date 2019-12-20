use crate::ast::choose_node::ChooseNode;
use crate::ast::node_type::NodeType::NString;
use crate::ast::string_node::StringNode;
use std::rc::Rc;
use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::ast::node::SqlNode;
use serde_json::json;
use crate::ast::config_holder::ConfigHolder;
use crate::engine::runtime::RbatisEngine;


#[test]
pub fn test_choose_node() {
    let mut holder= ConfigHolder::new();
    let mut john = json!({
        "arg": 2,
    });
    let engine= RbatisEngine::new();

    let s_node = NString(StringNode::new("dsaf#{arg+1}"));

    let mut c = ChooseNode {
        when_nodes: Option::Some(vec![s_node]),
        otherwise_node: None,
    };

    let r = c.eval(&mut john,&mut holder);
    println!("{}", r.unwrap());
}