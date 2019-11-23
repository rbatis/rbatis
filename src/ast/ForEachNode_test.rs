use crate::ast::ForEachNode::ForEachNode;
use crate::ast::Node::SqlNode;
use serde_json::json;
use crate::ast::NodeType::NodeType;
use crate::ast::StringNode::StringNode;
use std::rc::Rc;
use crate::ast::convert::SqlArgTypeConvertDefault::SqlArgTypeConvertDefault;
use crate::ast::NodeConfigHolder::NodeConfigHolder;


#[test]
pub fn TestForEachNode(){
    let mut holder=NodeConfigHolder::new();
    let mut n=ForEachNode{
        childs: vec![NodeType::NString(StringNode::new("index:#{index},item:#{item}"))],
        collection: "arg".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
        open: "(".to_string(),
        close: ")".to_string(),
        separator: ",".to_string()
    };
    let mut john = json!({
        "arg": 1,
    });

    let r=n.eval(&mut john,&mut holder);
    println!("{}", r.unwrap_or("null".to_string()));
}