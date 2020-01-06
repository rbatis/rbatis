use std::fs;
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value, Number};
use crate::ast::xml::bind_node::BindNode;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::node_type::NodeType;
use crate::example::activity::Activity;
use std::collections::LinkedList;
use crate::crud::ipage::IPage;


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
    //1 启用日志(可选，不添加则不加载日志库)
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    //2 初始化rbatis
    let mut rbatis = Rbatis::new();
    //3 加载数据库url name 为空，则默认数据库
    rbatis.load_db_url("".to_string(), "mysql://root:TEST@localhost:3306/test");
    //4 加载xml配置
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    //判断是否配置数据库
    let conf=rbatis.db_configs.get("").unwrap();
    if conf.db_addr.contains("localhost") {
        println!("请修改mysql链接'mysql://root:TEST@localhost:3306/test' 替换为具体的 用户名，密码，ip，和数据库名称");
        return;
    }
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data: Vec<Activity>= rbatis.eval("Example_ActivityMapper.xml", "select_by_condition", &mut json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    })).unwrap();
    // 写法2，直接运行原生sql
    // let data_opt: Result<serde_json::Value, String> = rbatis.eval_sql("select * from biz_activity");
    println!("[rbatis] result==>  {:?}", data);
}

#[test]
fn test_exec_select(){
    //1 启用日志(可选，不添加则不加载日志库)
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    //2 初始化rbatis
    let mut rbatis = Rbatis::new();
    //3 加载数据库url name 为空，则默认数据库
    rbatis.load_db_url("".to_string(), "mysql://root:TEST@localhost:3306/test");
    //4 加载xml配置
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    //判断是否配置数据库
    let conf=rbatis.db_configs.get("").unwrap();
    if conf.db_addr.contains("localhost") {
        println!("请修改mysql链接'mysql://root:TEST@localhost:3306/test' 替换为具体的 用户名，密码，ip，和数据库名称");
        return;
    }
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data:IPage<Activity> = rbatis.select_page("Example_ActivityMapper.xml", "select_by_condition", &mut json!({
       "name":"新人专享",
    }), &IPage::new(1,5)).unwrap();
    println!("[rbatis] result==>  {:?}", data);
}