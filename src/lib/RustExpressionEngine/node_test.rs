use crate::lib::RustExpressionEngine::node::{ Node};
use crate::lib::RustExpressionEngine::node::NodeType::{NString, NArg};
use serde_json::Value;
use serde_json::json;
use chrono::Local;
use crate::utils::time_util;
use serde_json::de::ParserNumber;
//use crate::lib::RustExpressionEngine::parser::{Parser,  ParserTokens};
use crate::lib::RustExpressionEngine::runtime::{IsNumber, OptMap};
use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use crate::lib::RustExpressionEngine::{runtime, parser};


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
    let john = json!({
        "a":1,
        "b":2,
        "c":"c",
    });
    let results = vec![
        Value::Bool(true),
        Value::String("fs".to_string()),
        Value::Bool(false),
        Value::Bool(true),
        Value::String("ac".to_string()),
        json!(2),
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
        println!("{}", item);
       //TODO let parserArray = Parser(item.to_string(), &OptMap::new());

        let (mut boxNode,_ )= parser::Parser(String::from(item), &OptMap::new());
        let result=boxNode.eval(&john);
        println!("result >>>>>>>>>>   =  {}", &result);
        let resultValue=&results[index];
        if !result.eq(resultValue){
           // println!("exe express fail:".to_owned()+item);
            panic!(">>>>>>>>>>>>>>>>>>>>>exe fail express:'".to_owned()+item+"'");
        }
        index += 1;
    }
}


#[test]
fn TestStringNode() {
    let mut strNode = Node::newString("sadf".to_string());
    strNode.eval(&Value::Null {});
    //println!("value:{}", result);
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

    let mut argNode = Node::newArg("sex.a".to_string());
     argNode.eval(&john);
    //println!("value:{},error:{}", result, Error);
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

    let mut argNode = Node::newArg("sex.a".to_string());

    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        argNode.eval(&john);
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
    let mut numb = Node::newNumberF64(1.02 as f64);
     numb.eval(&john);
   // println!("{}", value);
}

#[test]
fn TestBinaryNode() {
//    let john = json!({
//        "name": "John Doe",
//        "age": 1,
//         "sex":{
//            "a":"i'm a",
//            "b":"i'm b",
//         },
//        "phones": [
//            "+44 1234567",
//            "+44 2345678"
//        ]
//    });
//    let b = Node::newBinary("name".to_string(), String::new(), "+".to_string());
//    let (value, _) = b.Eval(&john);
//    println!("TestBinaryNode>>>>>>:{}", value);
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

    let  s = "'2019-02-26' == '2019-02-26'".to_string();

    let result= runtime::ParserTokens(&s);

    for item in result {
        println!("{}", item);
    }
}

#[test]
fn BenchmarkParserToken(){
    let  s = "'2019-02-26' == '2019-02-26'".to_string();


    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        runtime::ParserTokens(&s);
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}
