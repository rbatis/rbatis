use core::time;
use std::thread;
use std::thread::Thread;
use std::time::SystemTime;

use chrono::Local;
use serde_json::json;
use serde_json::Value;

use crate::engine::{parser, runtime};
use crate::engine::node::{Node, NodeType};
//use test::Bencher;
use crate::engine::node::NodeType::{NNumber, NOpt};
use crate::engine::runtime::OptMap;
use crate::utils;
use crate::utils::time_util;

#[test]
fn test_parser() {
    let box_node = parser::parse("-1 == -a", &OptMap::new()).unwrap();
    println!("{:#?}", &box_node);
    let john = json!({
        "a":1,
        "name": "John Doe",
        "age": {
           "yes":"sadf"
        },
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("result >>>>>>>>>>   =  {}", box_node.eval(&john).unwrap());
}

#[test]
fn test_benchmark() {
    let box_node = parser::parse("1<=2", &OptMap::new()).unwrap();
    let john = json!({
        "name": "John Doe",
    });
    let total = 10000000;
    let now = std::time::Instant::now();
    for _ in 0..total {
        for _ in 0..1 {
            box_node.clone();
            // box_node.clone();
            // n2.eval(&john);
        }
    }
    utils::time_util::print_each_time("test_benchmark", total, now);
    utils::time_util::print_qps("test_benchmark", total, now);

//    let ten_millis = time::Duration::from_secs(1000*60);
//    thread::sleep(ten_millis);
}

//#[bench]
//fn bench_parser_token(b: &mut Bencher) {
//    let optMap = OptMap::new();
//    let m= &OptMap::new();
//    let now=Local::now();
//    b.iter(|| {
//        runtime::parser_tokens(&String::from("n == 1"), &optMap);
//    });
//}
//
//#[bench]
//fn bench_parser(b: &mut Bencher) {
//    let m= &OptMap::new();
//    b.iter(|| {
//        parser::parser(String::from(" a + b"), m);
//    });
//}