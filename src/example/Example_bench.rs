use std::fs;
use crate::core::Rbatis::Rbatis;
use serde_json::{json, Value};
use crate::ast::BindNode::BindNode;
use crate::ast::Node::SqlNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;
use test::Bencher;
use chrono::Local;
use crate::utils;
use crate::ast::NodeType::NodeType;

struct Example{
   pub selectByCondition:fn()
}

#[bench]
fn BenchmarkExec(b: &mut Bencher) {
    let mut holder=NodeConfigHolder::new();
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    //println!("With text:/n{}", content);
    println!("start build -------------------------------------------------------");
    let mut rbatis=Rbatis::new(content);
    rbatis.print();

   // let mut node=rbatis.Get("selectByCondition");
//    println!("the node:{}",&node.print());

    let mut arg=json!({
       "name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
       "page":1,
       "size":1,
    });

    b.iter(|| {

        let mut node=rbatis.Get("selectByCondition").unwrap();
        let data=node.eval(&mut arg,&mut holder);
        if data.is_ok(){
           // println!("sql:{}",data.unwrap());
        }else{
            //println!("sql:fail={}",data.err().unwrap());
        }
    })
}

#[test]
fn TestBenchmarkTPS() {
    let now = Local::now();
    let total=100000;


    let mut holder=NodeConfigHolder::new();
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    //println!("With text:/n{}", content);
    println!("start build -------------------------------------------------------");
    let mut rbatis=Rbatis::new(content);
    rbatis.print();

    let mut node=rbatis.Get("selectByCondition").unwrap();
    println!("the node:{}",&node.print(0));

    let mut arg=json!({
       "name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
       "page":1,
       "size":1,
    });


    for _ in 0..total{
        node.eval(&mut arg,&mut holder);
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}