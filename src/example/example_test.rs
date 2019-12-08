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
    rbatis.set_db_url("".to_string(),"mysql://root:TEST@localhost:3306/test".to_string());//name 为空，则默认数据库
    rbatis.print();

    let mut arg=json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    });

    let data_opt:Result<serde_json::Value,String>=rbatis.eval("select_by_condition",&mut arg);

    if data_opt.is_ok(){
        let data=data_opt.unwrap();
        println!("result=========>is object {}",data.is_object());
        println!("result=========>is array {}",data.is_array());
        println!("result=========>{}",data);
    }else{
        println!("result=========>{}",data_opt.err().unwrap());
    }

}