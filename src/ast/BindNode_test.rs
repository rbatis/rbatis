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
        value: "1+1",
        engine: ExpressionEngineProxy::new(Rc::new(ExpressionEngineDefault::new()),
                                           ExpressionEngineCache::new()),
    };

    let mut john = json!({
        "a": 0,
    });


    let r=bindNode.eval(& mut john);


    println!("r={}",r);
    println!("john[a]={}",john["a"]);
}