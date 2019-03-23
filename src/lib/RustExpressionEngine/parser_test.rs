use crate::lib::RustExpressionEngine::parser;
use serde_json::json;
use serde_json::Value;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine::node::{Node, NodeItem2, NodeType, NumberNode, OptNode, Node2};
use crate::utils;
use chrono::Local;
use crate::utils::time_util;
use std::thread::Thread;
use test::Bencher;
use crate::lib::RustExpressionEngine::node::NodeType::{NNumber, NOpt};
use std::rc::Rc;


#[test]
fn TestParser() {
    let (boxNode,_ )= parser::Parser(String::from("1<=2"),&OptMap::new());
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
    let (v,_)=boxNode.Eval(&john);
    println!("item>>>>>>>>>>   =  {}", v);

}

#[test]
fn TestParser2() {
   let (n,e)= parser::Parser(String::from("a<=b+1-2>=1=='asdf + sdaf '"),&OptMap::new());

    println!("type:{}",n.Type())
}



#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let (boxNode,_ )= parser::Parser(String::from("1<=2"),&OptMap::new());
    let john = json!({
        "name": "John Doe",
    });
    let v=Value::String("sdf".to_string());

    let mut n2 =NodeItem2{
        Data: None,
        NArg: None,
        NString: None,
        NNumber: Option::Some(1 as f64),
        NBool: None,
        NNull: None,

        NBinaryLeft: Option::Some( Rc::new(NodeItem2{
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
        NBinaryRight: Option::Some(Rc::new(NodeItem2{
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
        //boxNode.Eval(&john);
       // boxNode.clone();
        n2.eval(&john);
    });
}