use crate::ast::StringNode::StringNode;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use std::rc::Rc;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
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

    let mut sNode = StringNode::new("arg+1=#{arg+1}", Box::new(NodeConfigHolder::new()));

    let r=sNode.eval(&mut john).unwrap();
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

    let mut sNode = StringNode::new("arg+1=#{arg}", Box::new(NodeConfigHolder::new()));

    b.iter(|| {
        sNode.eval(&mut john).unwrap();
    });
}