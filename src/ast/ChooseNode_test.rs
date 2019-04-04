use crate::ast::ChooseNode::ChooseNode;
use crate::ast::NodeType::NodeType::NString;
use crate::ast::StringNode::StringNode;
use std::rc::Rc;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::engines::ExpressionEngineProxy::ExpressionEngineProxy;
use crate::engines::ExpressionEngineDefault::ExpressionEngineDefault;
use crate::engines::ExpressionEngineCache::ExpressionEngineCache;
use crate::ast::NodeConfigHolder::NodeConfigHolder;


#[test]
pub fn TestChooseNode() {
    let mut john = json!({
        "arg": 2,
    });
    let engine=ExpressionEngineProxy::new(
        Rc::new(ExpressionEngineDefault::new()),
        ExpressionEngineCache::new());

    let sNode = NString(StringNode::new("dsaf#{arg+1}", Box::new(NodeConfigHolder::new())));

    let mut c = ChooseNode {
        whenNodes: Option::Some(vec![sNode]),
        otherwiseNode: None,
    };

    let r = c.eval(&mut john);
    println!("{}", r.unwrap());
}