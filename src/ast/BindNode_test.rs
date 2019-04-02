use crate::ast::BindNode::BindNode;
use std::rc::Rc;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::ast::Node::SqlNode;
use serde_json::json;

#[test]
fn TestBindNode(){
//    let bindNode=BindNode{
//        name: "".to_string(),
//        value: "".to_string(),
//        engine: Rc::new(ExpressionEngineProxy::newDefault()),
//    };
//
//    bindNode.eval(&json!("123321"));
}