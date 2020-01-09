use std::borrow::{Borrow, BorrowMut};
use std::cell::RefMut;
use std::collections::LinkedList;
use std::fs;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use log::{error, info, warn};
use rdbc::{DataType, Driver, ResultSet, ResultSetMetaData};
use rdbc_mysql::{MySQLDriver, MySQLResultSet};
use serde_json::{json, Number, Value};

use crate::ast::config_holder::ConfigHolder;
use crate::ast::xml::bind_node::BindNode;
use crate::ast::xml::node_type::NodeType;
use crate::core::rbatis::Rbatis;
use crate::crud::ipage::IPage;
use crate::decode::encoder::encode_to_value;
use crate::decode::rdbc_driver_decoder;
use crate::decode::rdbc_driver_decoder::decode_result_set;
use crate::example::activity::Activity;
use crate::example::conf::MYSQL_URL;

/**
 初始化实例
*/
fn init_rbatis() -> Result<Rbatis, String> {
    //1 启用日志(可选，不添加则不加载日志库)
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    //2 初始化rbatis
    let mut rbatis = Rbatis::new();
    //3 加载数据库url name 为空，则默认数据库
    rbatis.load_db_url("", MYSQL_URL);//"mysql://root:TEST@localhost:3306/test"
    //4 加载xml配置
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(), fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    //判断是否配置数据库
    let conf = rbatis.db_router.get("").unwrap();
    if conf.db_addr.contains("localhost") {
        error!("{}", "请修改mysql链接'mysql://root:TEST@localhost:3306/test' 替换为具体的 用户名，密码，ip，和数据库名称");
        return Err("请修改mysql链接'mysql://root:TEST@localhost:3306/test' 替换为具体的 用户名，密码，ip，和数据库名称".to_string());
    }

   //自定义动态数据源路由
   //    rbatis.router_func = Option::Some(|id| -> String{
   //        return "".to_string();
   //    });
    return Ok(rbatis);
}


/**
 示例-查询活动 数组 集合
*/
#[test]
fn test_exec_sql() {
    //初始化rbatis
    let rbatis = init_rbatis();
    if rbatis.is_err() {
        return;
    }
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data: Vec<Activity> = rbatis.unwrap().eval("Example_ActivityMapper.xml", "select_by_condition", &mut json!({
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

/**
分页查询数据
*/
#[test]
fn test_exec_select_page() {
    //初始化rbatis
    let rbatis = init_rbatis();
    if rbatis.is_err() {
        return;
    }
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data: IPage<Activity> = rbatis.unwrap().select_page("Example_ActivityMapper.xml", &mut json!({
       "name":"新人专享",
    }), &IPage::new(1, 5)).unwrap();
    println!("[rbatis] result==>  {:?}", data);
}

/**
   自定义分页查询数据(指定xml mapper id)
*/
#[test]
fn test_exec_select_page_custom() {
    //初始化rbatis
    let rbatis = init_rbatis();
    if rbatis.is_err() {
        return;
    }
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data: IPage<Activity> = rbatis.unwrap().select_page_by_mapper("Example_ActivityMapper.xml", "select_by_page", &mut json!({
       "name":"新人专享",
    }), &IPage::new(1, 5)).unwrap();
    println!("[rbatis] result==>  {:?}", data);
}

/**
   使用驱动直接执行sql
*/
#[test]
fn test_driver_row() {
    let driver = Arc::new(MySQLDriver::new());
    let mut conn = driver.connect(MYSQL_URL).unwrap();
    let mut stmt = conn.create("select * from biz_activity limit 1;").unwrap();
    let mut rs = stmt.execute_query(&vec![]).unwrap();//

    let (dd, size): (Result<Value, String>, usize) = decode_result_set(rs.as_mut());
    println!("{:?}", dd);
}

