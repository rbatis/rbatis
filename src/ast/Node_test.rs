use crate::ast::NodeType::NodeType;
use crate::ast::Node::Node;
use serde_json::{json, Value};
use crate::ast::StringNode::StringNode;
use test::Bencher;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;

#[test]
fn TestStringNode() {
    let john = json!({
        "name": "John Doe",
    });

    let convert=SqlArgTypeConvertDefault::new();
    let strNode = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}",Rc::new(convert)));

    let result = strNode.eval(&john);
    println!("{}", result);
}

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let john:&Value = &json!({
        "name": "John Doe",
    });
    let convert=SqlArgTypeConvertDefault::new();

    let strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv",Rc::new(convert)));

    b.iter(|| {
        &strNode.eval(john);
    });
}
