use crate::ast::NodeType::NodeType;
use crate::ast::Node::SqlNode;
use serde_json::{json, Value};
use crate::ast::StringNode::StringNode;
use test::Bencher;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[test]
fn TestStringNode() {
    let mut john = json!({
        "name": "John Doe",
    });
    let mut strNode = NodeType::NString(StringNode::new("select * from ${name} where name = #{name}", Box::new(NodeConfigHolder::new())));

    let result = strNode.eval(&mut john).unwrap();
    println!("{}", result);
}

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let mut john =  json!({
        "name": "John Doe",
    });


    let mut strNode = NodeType::NString(StringNode::new("vvvvvvvvvv#{name}vvvvvvvv", Box::new(NodeConfigHolder::new())));

    b.iter(|| {
        &strNode.eval(&mut john);
    });
}
