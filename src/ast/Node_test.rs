use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::json;
use crate::ast::StringNode::StringNode;

#[test]
fn TestStringNode() {
    let john = json!({
        "name": "John Doe",
    });

    let strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv"));

    let result = strNode.eval(john);
    println!("{}", result);
}
