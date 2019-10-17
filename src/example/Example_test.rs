use std::fs;
use crate::core::Rbatis::Rbatis;
use serde_json::Value;
use crate::ast::BindNode::BindNode;
use crate::ast::Node::SqlNode;

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
    let filePath = "./src/example/Example_ActivityMapper.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", filePath);
    let content = fs::read_to_string(filePath).unwrap();
    println!("With text:/n{}", content);
    println!("start build -------------------------------------------------------");
    let rbatis=Rbatis{};
    let mut node = rbatis.build(content);

    for x in node {
        let data= x.print();
        let data_str=data.as_str();
        println!("{}",data_str);
    }
//
//    let data=node.eval(&mut Value::String("".to_string()));
//    println!("data:{}",data.unwrap());
}