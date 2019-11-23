use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::{json, Value};
use crate::ast::StringNode::StringNode;
use test::Bencher;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;

use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::core::Rbatis::Rbatis;



#[test]
fn TestStringNode() {
    let mut holder=NodeConfigHolder::new();
    let mut john = json!({
        "name": "John Doe",
    });
    let mut strNode = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}"));

    let result = strNode.eval(&mut john,&mut holder).unwrap();
    println!("{}", result);
}

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let mut holder=NodeConfigHolder::new();
    let mut john =  json!({
        "name": "John Doe",
    });


    let mut strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv"));

    b.iter(|| {
        &strNode.eval(&mut john,&mut holder);
    });
}


