use crate::lib::RustExpressionEngine::node::{StringNode, Node, ArgNode};
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::Value;
use serde_json::json;
use chrono::Local;
use crate::utils::time_util;

#[test]
fn TestStringNode() {
    let strNode = StringNode {
        t: NString,
        value: String::from("asdfa"),
    };
    let (result, Error) = strNode.Eval(&Value::Null {});
    println!("value:{},error:{}", result, Error);
}

#[test]
fn TestArgNode() {
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let argNode = ArgNode::new("sex.a");
    let (result, Error) = argNode.Eval(&john);
    println!("value:{},error:{}", result, Error);
}

#[test]
fn BenchmarkArgNode() {
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let argNode = ArgNode::new("sex.a");

    let total=100000;
    let now=Local::now();
    for i in 0..total{
        argNode.Eval(&john);
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}