use crate::ast::ChooseNode::ChooseNode;
use crate::ast::NodeType::NodeType::NString;
use crate::ast::StringNode::StringNode;
use std::rc::Rc;
use crate::ast::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::ast::Node::SqlNode;
use serde_json::json;


#[test]
pub fn TestChooseNode() {
    let mut john = json!({
        "a": 1,
    });

    let sNode = NString(StringNode::new("dsaf", Rc::new(SqlArgTypeConvertDefault::new())));

    let mut c = ChooseNode {
        whenNodes: Option::Some(vec![sNode]),
        otherwiseNode: None,
    };

    let r = c.eval(&mut john);
    println!("{}", r.unwrap());
}