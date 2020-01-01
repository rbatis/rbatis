use serde_json::json;
use crate::ast::xml::if_node::IfNode;
use crate::ast::xml::string_node::StringNode;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node_type::NodeType;
use crate::ast::xml::node::SqlNode;
use std::rc::Rc;

#[test]
pub fn test_if_node() {
    let node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes"))],
        test: "arg == 1".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut holder= ConfigHolder::new();
    println!("{}", node.eval(&mut john,&mut holder).unwrap());
}
