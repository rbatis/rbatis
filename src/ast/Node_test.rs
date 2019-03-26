use crate::ast::NodeType::{NodeType};
use crate::ast::Node::Node;
use serde_json::json;
use crate::ast::StringNode::StringNode;

#[test]
fn TestStringNode(){
    let john = json!({
        "name": "John Doe",
    });

    let strNode=NodeType::NString(StringNode{
        value: "vvvvvvvvvvvvvvvvvvvvv".to_string(),
        expressMap: vec![],
        noConvertExpressMap: vec![]
    });

   let result=  strNode.eval(john);
    println!("{}",result);
}
