use crate::lib::RustExpressionEngine::parser;
use serde_json::json;
use serde_json::Value;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine::node::{Node,NodeType};
use crate::utils;
use chrono::Local;
use crate::utils::time_util;
use std::thread::Thread;
use test::Bencher;
use crate::lib::RustExpressionEngine::node::NodeType::{NNumber, NOpt};
use std::rc::Rc;
use std::sync::Arc;


#[test]
fn TestParser() {
    let (mut boxNode,_ )= parser::Parser(String::from("1 <= 2 "), &OptMap::new());
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
    boxNode.eval(&john);
    //println!("item>>>>>>>>>>   =  {}", v.);

}

#[test]
fn TestBenchmark() {
    let (mut boxNode,_ )= parser::Parser(String::from("1<=2"), &OptMap::new());
    let john = json!({
        "name": "John Doe",
    });
    let v=Value::String("sdf".to_string());

    let mut n2 =Node{
        Data: None,
        NArg: None,
        NString: None,
        NNumber: Option::Some(1 as f64),
        NBool: None,
        NNull: None,

        NBinaryLeft: Option::Some( Arc::new(Node{
            Data: None,
            NArg: None,
            NString: None,
            NNumber: Option::Some(1 as f64),
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNumber)
        })),
        NBinaryRight: Option::Some(Arc::new(Node{
            Data: None,
            NArg: None,
            NString: None,
            NNumber: Option::Some(1 as f64),
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNumber)
        })),
        NOpt: Option::Some("<=".to_string()),
        t: Option::Some(NodeType::NNumber),
    };
    let v=n2.eval(&john);
    println!("{}",v.NNumber.unwrap());
    let total=100000;
    let now=Local::now();
    for i in 0..total{
        boxNode.eval(&john);
        // boxNode.clone();
       // n2.eval(&john);
    }
    utils::time_util::count_time(total,now);
}



#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let (mut boxNode,_ )= parser::Parser(String::from("1<=2"), &OptMap::new());
    let john = json!({
        "name": "John Doe",
    });
    let v=Value::String("sdf".to_string());

    let mut n2 =Node{
        Data: None,
        NArg: None,
        NString: None,
        NNumber: Option::Some(1 as f64),
        NBool: None,
        NNull: None,

        NBinaryLeft: Option::Some( Arc::new(Node{
            Data: None,
            NArg: None,
            NString: None,
            NNumber: Option::Some(1 as f64),
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNumber)
        })),
        NBinaryRight: Option::Some(Arc::new(Node{
            Data: None,
            NArg: None,
            NString: None,
            NNumber: Option::Some(1 as f64),
            NBool: None,
            NNull: None,
            NBinaryLeft: None,
            NBinaryRight: None,
            NOpt: None,
            t: Option::Some(NNumber)
        })),
        NOpt: Option::Some("<=".to_string()),
        t: Option::Some(NodeType::NNumber),
    };
    let v=n2.eval(&john);
    println!("{}",v.NNumber.unwrap());

    let now=Local::now();
    b.iter(|| {
        //boxNode.eval(&john);
       // boxNode.clone();
        n2.eval(&john);
    });
}