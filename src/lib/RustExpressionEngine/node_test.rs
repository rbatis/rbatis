use crate::lib::RustExpressionEngine::node::{StringNode, Node, ArgNode, NumberNode, BinaryNode};
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::Value;
use serde_json::json;
use chrono::Local;
use crate::utils::time_util;
use serde_json::de::ParserNumber;
use crate::lib::RustExpressionEngine::parser::Parser;
use crate::lib::RustExpressionEngine::runtime::IsNumber;


#[test]
fn TestString(){
    let s=String::from("'asd'");

    let firstIndex=s.find("'").unwrap_or_default();
    let lastIndex=s.rfind("'").unwrap_or_default();

    if firstIndex==0&&lastIndex==(s.len()-1) && firstIndex!=lastIndex{
        println!("yes")
    }

    let s2=String::from("123.44");
    println!("{}",IsNumber(&s2));
}

#[test]
fn TestNodeRun() {
    let results = vec![
        Value::Bool(true),
        Value::String("fs".to_string()),
        Value::Bool(false),
        Value::Bool(true),
        Value::String("ac".to_string()),
        Value::Number(serde_json::Number::from(ParserNumber::I64(2))),
        Value::Bool(false),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(true),
    ];

    let expressions = vec!["'2019-02-26' == '2019-02-26'",
                           "`f`+`s`",
                           "a +1 > b * 8",
                           "a >= 0",
                           "'a'+c",
                           "b",
                           "a < 1",
                           "a +1 > b*8",
                           "a * b == 2",
                           "a - b == 0",
                           "a >= 0 && a != 0",
                           "a == 1 && a != 0",
                           "1 > 3 ",
                           "1 + 2 != nil",
                           "1 != null",
                           "1 + 2 != nil && 1 > 0 ",
                           "1 + 2 != nil && 2 < b*8 ", ];

    let mut index = 0;
    for item in expressions {
        index += 1;
        println!("{}", item);
        let parserArray = Parser(item.to_string());

    }
}


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

    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        argNode.Eval(&john);
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}

#[test]
fn TestNumberNode() {
    let john = json!({
        "name": "John Doe",
        "age": 1,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let numb = NumberNode::new(&String::from("1.02"));
    let (value, _) = numb.Eval(&john);
    println!("{}", value);
}

#[test]
fn TestBinaryNode() {
    let john = json!({
        "name": "John Doe",
        "age": 1,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    let l = StringNode::new("name".to_string());
    let r = StringNode::new("name".to_string());
    let b = BinaryNode::new(l, r, "+".to_string());
    let (value, _) = b.Eval(&john);
    println!("{}", value);
}