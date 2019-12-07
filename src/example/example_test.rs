use std::fs;
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value};
use crate::ast::bind_node::BindNode;
use crate::ast::node::SqlNode;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::node_type::NodeType;

struct Example{
   pub select_by_condition:fn()
}


#[test]
fn testWriteMethod(){
    let e=Example{
        select_by_condition: || {println!("select * from table");}
    };
    (e.select_by_condition)();
}


#[test]
fn testLoadXml(){
    let mut holder= ConfigHolder::new();
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

    let data=rbatis.eval("select_by_condition",&mut arg);
    if data.is_ok(){
        println!("sql:{}",data.unwrap());
    }else{
        println!("sql:fail={}",data.err().unwrap());
    }
}