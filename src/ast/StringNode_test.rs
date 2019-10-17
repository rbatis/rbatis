use crate::ast::StringNode::StringNode;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use std::rc::Rc;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::lib::RustExpressionEngine::node::NodeType::NString;
use crate::ast::Node::SqlNode;
use serde_json::json;
use test::Bencher;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[test]
pub fn TestStringNode(){
    let mut john = json!({
        "arg": 2,
    });
    let mut holder=NodeConfigHolder::new();
    let mut sNode = StringNode::new("arg+1=#{arg+1}");

    let r=sNode.eval(&mut john,&mut holder).unwrap();
    println!("{}",r);
}


#[bench]
fn Bench_StringNode(b: &mut Bencher) {
    let mut john = json!({
        "arg": 2,
    });

    let engine=ExpressionEngineProxy::new(
        Rc::new(ExpressionEngineDefault::new()),
        ExpressionEngineCache::new());

    let mut sNode = StringNode::new("arg+1=#{arg}");
    let mut holder=NodeConfigHolder::new();
    b.iter(|| {
        sNode.eval(&mut john,&mut holder).unwrap();
    });
}