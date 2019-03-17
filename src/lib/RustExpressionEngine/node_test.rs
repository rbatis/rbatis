use crate::lib::RustExpressionEngine::node::{StringNode, Node, ArgNode};
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::Value;
use serde_json::json;

#[test]
fn TestStringNode() {
    let strNode = StringNode {
        t: NString,
        value: String::from("asdfa"),
    };
    let (result, Error) = strNode.Eval(Value::Null {});
    println!("value:{},error:{}", result, Error);
}

#[test]
fn TestArgNode() {
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let argNode = ArgNode {
        t: NArg,
        value: String::from("name"),
    };
    let (result, Error) = argNode.Eval(john);
    println!("value:{},error:{}", result, Error);
}