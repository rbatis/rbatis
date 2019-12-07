use serde_json::json;
use crate::ast::if_node::IfNode;
use crate::ast::string_node::StringNode;
use crate::ast::node_config_holder::NodeConfigHolder;
use crate::ast::node_type::NodeType;
use crate::ast::node::SqlNode;
use std::rc::Rc;

#[test]
pub fn test_if_node() {
    let mut node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes"))],
        test: "arg == 1".to_string(),
    };
    let mut john = json!({
        "arg": 1,
    });
    let mut holder=NodeConfigHolder::new();
    println!("{}", node.eval(&mut john,&mut holder).unwrap());
}
