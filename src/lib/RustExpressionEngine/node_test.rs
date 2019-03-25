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

#[derive(Clone, PartialEq)]
struct  Eq{
   pub express:String,
   pub eq:Value,
}

#[test]
fn TestNodeRun() {
    let john = json!({
        "a":1,
        "b":2,
        "c":"c",
    });
    let expressions:Vec<Eq>=vec![
            Eq{express: "'2019-02-26' == '2019-02-26'".to_string(),eq: json!(true)},
            Eq{express: "`f`+`s`".to_string(),eq: json!("fs")},
            Eq{express: "a +1 > b * 8".to_string(),eq: json!(false)},
            Eq{express: "a >= 0".to_string(),eq: json!(true)},
            Eq{express:  "'a'+c".to_string(),eq: json!("ac")},
            Eq{express: "b".to_string(),eq: json!(2)},
            Eq{express: "a < 1".to_string(),eq: json!(false)},
            Eq{express: "a +1 > b*8".to_string(),eq: json!(false)},
            Eq{express: "a * b == 2".to_string(),eq: json!(true)},
            Eq{express:  "a - b == 0".to_string(),eq: json!(false)},
            Eq{express:  "a >= 0 && a != 0".to_string(),eq: json!(true)},
            Eq{express: "a == 1 && a != 0".to_string(),eq: json!(true)},
            Eq{express: "1 > 3 ".to_string(),eq: json!(false)},
            Eq{express: "1 + 2 != nil".to_string(),eq: json!(true)},
            Eq{express: "1 != null".to_string(),eq: json!(true)},
            Eq{express: "1 + 2 != nil && 1 > 0 ".to_string(),eq: json!(true)},
            Eq{express: "1 + 2 != nil && 2 < b*8 ".to_string(),eq: json!(true)},
    ];


    let mut index = 0;
    for item in expressions {
        println!("{}", item.express.clone());
       //TODO let parserArray = Parser(item.to_string(), &OptMap::new());
        let (mut boxNode,_ )= parser::Parser(item.express.clone(), &OptMap::new());
        let result=boxNode.eval(&john);
        println!("result >>>>>>>>>>   =  {}", &result);
        let resultValue=&item.eq.clone();
        if !result.eq(resultValue){
           // println!("exe express fail:".to_owned()+item);
            panic!(">>>>>>>>>>>>>>>>>>>>>exe fail express:'".to_owned()+item.clone().express.as_str()+"'");
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
