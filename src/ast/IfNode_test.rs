use serde_json::json;
use crate::ast::IfNode::IfNode;
use crate::ast::StringNode::StringNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use std::rc::Rc;

#[test]
pub fn TestIfNode() {
    let h = NodeConfigHolder::new();
    let mut node = IfNode {
        childs: vec![NodeType::NString(StringNode::new("yes", h.clone()))],
        test: "arg == 1".to_string(),
        holder: h,
    };
    let mut john = json!({
        "arg": 1,
    });
    println!("{}", node.eval(&mut john).unwrap());
}
