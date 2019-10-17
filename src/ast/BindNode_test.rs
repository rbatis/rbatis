use crate::ast::BindNode::BindNode;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[test]
fn TestBindNode(){
    let mut holder=NodeConfigHolder::new();
    let mut bindNode =BindNode{
        name: "a".to_string(),
        value: "a+1".to_string(),
    };

    let mut john = json!({
        "a": 1,
    });


    let r=bindNode.eval(& mut john,&mut holder).unwrap();


    println!("r={}",r);
    println!("john[a]={}",john["a"]);
}