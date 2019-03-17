use crate::lib::RustExpressionEngine::node::{StringNode, Node};
use crate::lib::RustExpressionEngine::node::NodeType::NString;
use serde_json::Value;

#[test]
fn TestStringNode() {
    let strNode = StringNode {
        t: NString,
        value: String::from("asdfa"),
    };
    let (result, Error) = strNode.Eval(Value::Null {});
    println!("value:{},error:{}", result, Error);
}