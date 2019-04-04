use crate::ast::ForEachNode::ForEachNode;
use crate::ast::Node::SqlNode;
use serde_json::json;


#[test]
pub fn TestForEachNode(){
    let mut n=ForEachNode{
        childs: vec![],
        collection: "".to_string(),
        index: "".to_string(),
        item: "".to_string(),
        open: "".to_string(),
        close: "".to_string(),
        separator: "".to_string()
    };
    let mut john = json!({
        "arg": 2,
    });

    let r=n.eval(&mut john);
    println!("{}", r.unwrap());
}