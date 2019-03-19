use crate::lib::RustExpressionEngine::node::{StringNode, Node, ArgNode, NumberNode, BinaryNode};
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::Value;
use serde_json::json;
use chrono::Local;
use crate::utils::time_util;
use serde_json::de::ParserNumber;
use crate::lib::RustExpressionEngine::parser::{Parser, OptMap, ParserTokens};
use crate::lib::RustExpressionEngine::runtime::IsNumber;
use std::collections::HashMap;
use std::collections::linked_list::LinkedList;


#[test]
fn TestString() {
    let s = String::from("'asd'");

    let firstIndex = s.find("'").unwrap_or_default();
    let lastIndex = s.rfind("'").unwrap_or_default();

    if firstIndex == 0 && lastIndex == (s.len() - 1) && firstIndex != lastIndex {
        println!("yes")
    }

    let s2 = String::from("123.44");
    println!("{}", IsNumber(&s2));
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
        let parserArray = Parser(item.to_string(), &OptMap::new());
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

    let argNode = ArgNode::new(&"sex.a".to_string());
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

    let argNode = ArgNode::new(&"sex.a".to_string());

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
    let b = BinaryNode::new(Box::new(l), Box::new(r), "+".to_string());
    let (value, _) = b.Eval(&john);
    println!("{}", value);
}

#[test]
fn TestMatcher() {

//    vec!["'2019-02-26' == '2019-02-26'",
//         "`f`+`s`",
//         "a +1 > b * 8",
//         "a >= 0",
//         "'a'+c",
//         "b",
//         "a < 1",
//         "a +1 > b*8",
//         "a * b == 2",
//         "a - b == 0",
//         "a >= 0 && a != 0",
//         "a == 1 && a != 0",
//         "1 > 3 ",
//         "1 + 2 != nil",
//         "1 != null",
//         "1 + 2 != nil && 1 > 0 ",
//         "1 + 2 != nil && 2 < b*8 ", ];





    let mut s = "'a+b'=='c'".to_string();


    let chars = s.chars();

    println!("{}", "================================");

    //let mut itemMap = HashMap::new();
    let mut find = false;

    let mut result = vec![];

    let mut temp = String::new();
    for item in chars {
        //println!("{}", item);

        if item == '\'' {
            if find {
                //第二次找到
                find = false;
                temp.push(item);
                result.push(temp.clone());
                println!("{}", temp);

                temp = String::new();
                continue;
            }
            find = true;
            temp.push(item);
            continue;
        }
        if find {
            temp.push(item);
        }
    }

    let mut i = 0;
    for item in &result.clone() {
        let mut strItem = String::from("`");
        strItem.push_str(i.to_string().as_str());
        s = s.replace(item.as_str(), strItem.as_str());
        i = i + 1;
    }

    println!("{}", s);
}


#[test]
fn TestMatcher2() {
//"'a+b'=='c'"
//    vec!["'2019-02-26' == '2019-02-26'",
//         "`f`+`s`",
//         "a +1 > b * 8",
//         "1 != null",
//         "1 + 2 != nil && 1 > 0 ",
//         "1 + 2 != nil && 2 < b*8 ", ];
    let optMap = OptMap::new();

    let mut s = "'2019-02-26' == '2019-02-26'".to_string();

    let result= ParserTokens(&s);

    for item in result {
        println!("{}", item);
    }
}
