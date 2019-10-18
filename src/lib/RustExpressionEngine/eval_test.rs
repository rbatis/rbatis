use crate::lib::RustExpressionEngine::parser;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use chrono::Local;
use serde_json::json;
use test::Bencher;
use std::{time, thread};

#[test]
fn TestEvalArg() {
    let mut boxNode= parser::Parser(String::from("startTime == null"), &OptMap::new()).unwrap();
    let john = json!({
        "n":1,
        "name": "John Doe",
        "startTime":"1",
        "endTime":"1",
         "age": {
           "yes":"sadf"
        }
    });
    let v=boxNode.eval(&john).unwrap();
    println!("{:?}",v);
}


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

    let total=10000000;
    println!("start");

    for _loop in 0..3{
        for i in 0..total{
            boxNode.eval(&john);
            if i==(total-1){
                println!("done:{}",_loop);
                let ten_millis = time::Duration::from_secs(5);
                thread::sleep(ten_millis);
            }
            if i%1000000==0{
                println!("number:{}",i)
            }
        }
    }
}