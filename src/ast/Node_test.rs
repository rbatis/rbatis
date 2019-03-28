use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::{json, Value};
use crate::ast::StringNode::StringNode;
use test::Bencher;

#[test]
fn TestStringNode() {
    let john = json!({
        "name": "John Doe",
    });

    let strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv${name}"));

    let result = strNode.eval(&john);
    println!("{}", result);
}

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let john:&Value = &json!({
        "name": "John Doe",
    });

    let strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv"));

    b.iter(|| {
        &strNode.eval(john);
    });
}
