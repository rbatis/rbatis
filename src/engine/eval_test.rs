//use test::Bencher;
use std::{thread, time};

use chrono::Local;
use serde_json::json;

use crate::engine::node::Node;
use crate::engine::parser;
use crate::engine::runtime::OptMap;

#[test]
fn test_eval_arg() {
    let box_node = parser::parse("-1 == -1", &OptMap::new()).unwrap();
    println!("{:#?}", box_node);
    let john = json!({
    });
    let v = box_node.eval(&john).unwrap();
    println!("{:?}", v);
}


//#[bench]
//fn bench_parser(b: &mut Bencher) {
//    let mut boxNode= parser::parser(String::from("'1'+'1'"), &OptMap::new()).unwrap();
//    let john = json!({
//        "n":1,
//        "name": "John Doe",
//         "age": {
//           "yes":"sadf"
//        }
//    });
//    b.iter(|| {
//        boxNode.eval(&john);
//    });
//}

#[test]
fn test_mem_gc() {
    let box_node: Node = parser::parse("'1'+'1'", &OptMap::new()).unwrap();
    let john = json!({
        "n":1,
        "name": "John Doe",
         "age": {
           "yes":"sadf"
        }
    });

    let total = 10000000;
    println!("start");

    for _loop in 0..3 {
        for i in 0..total {
            box_node.eval(&john);
            if i == (total - 1) {
                println!("done:{}", _loop);
                let ten_millis = time::Duration::from_secs(5);
                thread::sleep(ten_millis);
            }
            if i % 1000000 == 0 {
                println!("number:{}", i)
            }
        }
    }
}