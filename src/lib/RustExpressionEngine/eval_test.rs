use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use chrono::Local;
use serde_json::json;
use test::Bencher;
use std::{time, thread};

#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let mut boxNode= parser::Parser(String::from("'1'+'1'"), &OptMap::new()).unwrap();
    let john = json!({
        "n":1,
        "name": "John Doe",
         "age": {
           "yes":"sadf"
        }
    });
    let now=Local::now();
    b.iter(|| {
        boxNode.eval(&john);
    });
}

#[test]
fn TestMemGC() {
    let mut boxNode= parser::Parser(String::from("'1'+'1'"), &OptMap::new()).unwrap();
    let john = json!({
        "n":1,
        "name": "John Doe",
         "age": {
           "yes":"sadf"
        }
    });

    let total=1000000;
    println!("start");
    for i in 0..total{
        boxNode.eval(&john);
        if i==(total-1){
            println!("999999");
            let ten_millis = time::Duration::from_secs(5);
            thread::sleep(ten_millis);
        }
    }
    for i in 0..total{
        boxNode.eval(&john);
        if i==(total-1){
            println!("999999");
            let ten_millis = time::Duration::from_secs(5);
            thread::sleep(ten_millis);
        }
    }
    for i in 0..total{
        boxNode.eval(&john);
        if i==(total-1){
            println!("999999");
            let ten_millis = time::Duration::from_secs(5);
            thread::sleep(ten_millis);
        }
    }
}