use crate::ast::TrimNode::TrimNode;
use serde_json::json;
use crate::ast::Node::SqlNode;
use crate::ast::NodeType::NodeType;
use crate::ast::StringNode::StringNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

#[test]
pub fn TestTrimNode(){
    let mut node =TrimNode{
        childs: vec![NodeType::NString(StringNode::new("1trim value1",NodeConfigHolder::new()))],
        prefix: "(".to_string(),
        suffix: ")".to_string(),
        suffixOverrides: "1".to_string(),
        prefixOverrides: "1".to_string()
    };
    let mut john = json!({
        "arg": 2,
    });

   let r= node.eval(&mut john).unwrap();
    println!("{}",r)
}