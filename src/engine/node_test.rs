use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::time::SystemTime;

use chrono::Local;
use serde_json::json;
use serde_json::Value;

use crate::engine::{parser, runtime};
use crate::engine::node::Node;
use crate::engine::node::NodeType::{NArg, NString};
//use crate::engines::RustExpressionEngine::parser::{parser,  ParserTokens};
use crate::engine::runtime::{is_number, OptMap};
use crate::utils::time_util;

//use test::Bencher;

#[derive(Clone, PartialEq)]
struct Eq<'a> {
    pub express: &'a str,
    pub eq: Value,
}

#[test]
fn test_node_run() {
    let john = json!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
    });
    let expressions: Vec<Eq> = vec![
        Eq { express: "d.a == null", eq: json!(true) },
        Eq { express: "'2019-02-26' == '2019-02-26'", eq: json!(true) },
        Eq { express: "`f`+`s`", eq: json!("fs") },
        Eq { express: "a +1 > b * 8", eq: json!(false) },
        Eq { express: "a >= 0", eq: json!(true) },
        Eq { express: "'a'+c", eq: json!("ac") },
        Eq { express: "b", eq: json!(2) },
        Eq { express: "a < 1", eq: json!(false) },
        Eq { express: "a +1 > b*8", eq: json!(false) },
        Eq { express: "a * b == 2", eq: json!(true) },
        Eq { express: "a - b == 0", eq: json!(false) },
        Eq { express: "a >= 0 && a != 0", eq: json!(true) },
        Eq { express: "a == 1 && a != 0", eq: json!(true) },
        Eq { express: "1 > 3 ", eq: json!(false) },
        Eq { express: "1 + 2 != nil", eq: json!(true) },
        Eq { express: "1 != null", eq: json!(true) },
        Eq { express: "1 + 2 != nil && 1 > 0 ", eq: json!(true) },
        Eq { express: "1 + 2 != nil && 2 < b*8 ", eq: json!(true) },
        Eq { express: "-1 != nil", eq: json!(true) },
        Eq { express: "-1 != -2 && -1 == 2-3 ", eq: json!(true) },
        Eq { express: "-1 == a*-1 ", eq: json!(true) },
        Eq { express: "-1 + a*-1 ", eq: json!(-2.0) },
    ];


    let mut index = 0;
    for item in expressions {
        println!("{}", item.express.clone());
        let box_node = parser::parse(item.express, &OptMap::new()).unwrap();
        let result = box_node.eval(&john).unwrap();
        println!("express: {} >>>>> {}", item.express, &result);
        let result_value = &item.eq.clone();
        if !result.eq(result_value) {
            // println!("exe express fail:".to_owned()+item);
            panic!("[rbatis] >>>>>>>>>>>>>>>>>>>>>exe fail express:'{}',result:{}",&item.express,&result);
        }
        index += 1;
    }
}


#[test]
fn test_string_node() {
    let str_node = Node::new_string("sadf");
    str_node.eval(&Value::Null {});
    //println!("value:{}", result);
}

#[test]
fn test_arg_node() {
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

    let arg_node = Node::new_arg("sex.a");
    arg_node.eval(&john);
    //println!("value:{},error:{}", result, Error);
}

#[test]
fn benchmark_arg_node() {
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

    let arg_node = Node::new_arg("sex.a");

    let total = 100000;
    let now = std::time::Instant::now();
    for i in 0..total {
        arg_node.eval(&john);
    }
    time_util::count_time_qps("benchmark_arg_node", total, now);
}

#[test]
fn test_number_node() {
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
    let numb = Node::new_number_f64(1.02 as f64);
    numb.eval(&john);
    // println!("{}", value);
}

#[test]
fn benchmark_parser_token() {
    let s = "'2019-02-26' == '2019-02-26'".to_string();
    let opt_map = OptMap::new();

    let total = 100000;
    let now = std::time::Instant::now();
    for i in 0..total {
        runtime::parser_tokens(&s, &opt_map);
    }
    time_util::count_time_qps("benchmark_parser_token", total, now);
}


//#[bench]
//fn bench_node_eval(b: &mut Bencher) {
//    let rc=Rc::new("asdf".to_string());
//    b.iter(|| {
//        rc.clone();
//    });
//}