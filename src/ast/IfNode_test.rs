use serde_json::json;
use crate::ast::IfNode::IfNode;
use crate::ast::StringNode::StringNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use std::rc::Rc;

#[test]
pub fn TestIfNode() {
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
