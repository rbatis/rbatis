use crate::lib::RustExpressionEngine::parser;
use serde_json::json;
use serde_json::Value;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine::node::Node;
use crate::utils;
use chrono::Local;
use crate::utils::time_util;
use std::thread::Thread;


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

#[test]
fn BenchmarkParser(){
    let (boxNode,_ )= parser::Parser(String::from("1<=2"),&OptMap::new());
    let john = json!({
        "name": "John Doe",
    });

    let total=100000;

    let v=Value::String("sdf".to_string());

    let now=Local::now();

    for item in 0..total{

        for i in 0..1{
            boxNode.Eval(&john);
            //boxNode.clone();
          //  boxNode.clone();
        }
    }

    time_util::count_time(total, now);
}