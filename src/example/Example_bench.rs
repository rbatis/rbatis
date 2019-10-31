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


#[bench]
fn testLoadXmlBench(b: &mut Bencher) {
    let mut holder=NodeConfigHolder::new();
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    //println!("With text:/n{}", content);
    println!("start build -------------------------------------------------------");
    let mut rbatis=Rbatis::new(content);
    rbatis.print();


    let mut arg=json!({
       "name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
       "page":1,
       "size":1,
    });

    b.iter(||{
        let data=rbatis.eval("selectByCondition",&mut arg);
    })
}
