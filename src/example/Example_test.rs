use std::fs;
use crate::core::Rbatis::Rbatis;
use serde_json::{json, Value};
use crate::ast::BindNode::BindNode;
use crate::ast::Node::SqlNode;
use crate::ast::NodeConfigHolder::NodeConfigHolder;

struct Example{
   pub selectByCondition:fn()
}


#[test]
fn testWriteMethod(){
    let e=Example{
        selectByCondition: || {println!("select * from table");}
    };
    (e.selectByCondition)();
}


#[test]
fn testLoadXml(){
    let mut holder=NodeConfigHolder::new();
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    //println!("With text:/n{}", content);
    println!("start build -------------------------------------------------------");
    let mut rbatis=Rbatis::new(content);
    rbatis.print();

    let mut node=rbatis.Get("selectByCondition");
    println!("the node:{}",&node.print());

    let mut arg=json!({
        //"name":"sadf",
       "startTime":"startTime",
       "endTime":"endTime",
    });

    let data=node.eval(&mut arg,&mut holder);
    if data.is_ok(){
        println!("sql:{}",data.unwrap());
    }else{
        println!("sql:fail={}",data.err().unwrap());
    }
}