use crate::ast::ChooseNode::ChooseNode;
use crate::ast::NodeType::NodeType::NString;
use crate::ast::StringNode::StringNode;
use std::rc::Rc;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use crate::engines::RbatisEngine::runtime::RbatisEngine;


#[test]
pub fn TestChooseNode() {
    let mut holder=NodeConfigHolder::new();
    let mut john = json!({
        "arg": 2,
    });
    let engine= RbatisEngine::new();

    let sNode = NString(StringNode::new("dsaf#{arg+1}"));

    let mut c = ChooseNode {
        whenNodes: Option::Some(vec![sNode]),
        otherwiseNode: None,
    };

    let r = c.eval(&mut john,&mut holder);
    println!("{}", r.unwrap());
}