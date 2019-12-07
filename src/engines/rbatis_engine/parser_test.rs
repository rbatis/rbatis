use crate::engines::rbatis_engine::{parser, runtime};
use serde_json::json;
use serde_json::Value;
use crate::engines::rbatis_engine::runtime::OptMap;
use crate::engines::rbatis_engine::node::{Node, NodeType};
use crate::utils;
use chrono::Local;
use crate::utils::time_util;
use std::thread::Thread;
use test::Bencher;
use crate::engines::rbatis_engine::node::NodeType::{NNumber, NOpt};
use std::rc::Rc;
use std::sync::Arc;
use core::time;
use std::thread;


#[test]
fn TestParser() {
    let mut boxNode= parser::parser(String::from("a == 1 && a != 0"), &OptMap::new()).unwrap();
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
    println!("result >>>>>>>>>>   =  {}", boxNode.eval(&john).unwrap());

}

#[test]
fn TestBenchmark() {
    let mut boxNode= parser::parser(String::from("1<=2"), &OptMap::new()).unwrap();
    let john = json!({
        "name": "John Doe",
    });
    let total=10000000;
    let now=Local::now();
    for _ in 0..total{
        for _ in 0..1{
            boxNode.clone();
            // boxNode.clone();
            // n2.eval(&john);
        }
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);

//    let ten_millis = time::Duration::from_secs(1000*60);
//    thread::sleep(ten_millis);
}

#[bench]
fn bench_parser_token(b: &mut Bencher) {
    let optMap = OptMap::new();
    let m= &OptMap::new();
    let now=Local::now();
    b.iter(|| {
        runtime::parser_tokens(&String::from("n == 1"), &optMap);
    });
}

#[bench]
fn bench_parser(b: &mut Bencher) {
    let m= &OptMap::new();
    b.iter(|| {
        parser::parser(String::from(" a + b"), m);
    });
}