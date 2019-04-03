use crate::ast::BindNode::BindNode;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;

#[test]
fn TestBindNode(){
    let mut bindNode =BindNode{
        name: "a",
        value: "a+1",
        engine: ExpressionEngineProxy::new(Rc::new(ExpressionEngineDefault::new()),
                                           ExpressionEngineCache::new()),
    };

    let mut john = json!({
        "a": 1,
    });


    let r=bindNode.eval(& mut john).unwrap();


    println!("r={}",r);
    println!("john[a]={}",john["a"]);
}