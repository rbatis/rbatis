use crate::ast::choose_node::ChooseNode;
use crate::ast::node_type::NodeType::NString;
use crate::ast::string_node::StringNode;
use std::rc::Rc;
use crate::ast::convert::sql_arg_type_convert_default::SqlArgTypeConvertDefault;
use crate::ast::node::SqlNode;
use serde_json::json;
use crate::ast::node_config_holder::NodeConfigHolder;
use crate::engines::RbatisEngine::runtime::RbatisEngine;


#[test]
pub fn TestChooseNode() {
    let mut holder=NodeConfigHolder::new();
    let mut john = json!({
        "arg": 2,
    });
    let engine= RbatisEngine::new();

    let sNode = NString(StringNode::new("dsaf#{arg+1}"));

    let mut c = ChooseNode {
        whenNodes: Option::Some(vec![sNode]),
        otherwiseNode: None,
    };

    let r = c.eval(&mut john,&mut holder);
    println!("{}", r.unwrap());
}