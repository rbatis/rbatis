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
fn test_write_method(){
    let e=Example{
        select_by_condition: || {println!("select * from table");}
    };
    (e.select_by_condition)();
}


#[test]
fn test_exec_sql(){
    println!("start build -------------------------------------------------------");
    let mut rbatis=Rbatis::new();
    let url="mysql://root:TEST@localhost:3306/test";
    rbatis.load_db_url("".to_string(), url.to_string());//name 为空，则默认数据库

    if url.contains("localhost"){
        println!("请修改mysql链接 用户名，密码，ip，和数据库名称");
        return;
    }
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(),fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.print();//打印已读取的内容
    println!(">>>>>>>>>>>>>>>>>>>>>>start eval method >>>>>>>>>>>>>>>>>>>>>>>");
    //执行到远程mysql 并且获取结果
    let data_opt:Result<serde_json::Value,String>=rbatis.eval("Example_ActivityMapper.xml".to_string(),"select_by_condition",&mut json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    }));
    println!(">>>>>>>>>>>>>>>>>>>>>>get result>>>>>>>>>>>>>>>>>>>>>>>");
    if data_opt.is_ok(){
        let data=data_opt.unwrap();
        println!("result=========>{}",data);
    }else{
        println!("result=========>{}",data_opt.err().unwrap());
    }
    println!(">>>>>>>>>>>>>>>>>>>>>> eval done >>>>>>>>>>>>>>>>>>>>>>>");
}