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

fn init_rbatis()->Result<Rbatis,String>{
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
        return Err("请修改mysql链接'mysql://root:TEST@localhost:3306/test' 替换为具体的 用户名，密码，ip，和数据库名称".to_string());
    }
    return Ok(rbatis);
}



/**
 示例-查询活动 数组 集合

2020-01-06T16:40:14.106240+08:00 INFO rbatis::core::rbatis - [rbatis] Query ==>  select * from biz_activity  order by create_time desc
2020-01-06T16:40:14.233951+08:00 INFO rbatis::core::rbatis - [rbatis] ReturnRows <== 2
[rbatis] result==>  [Activity { id: Some("\"dfbdd779-5f70-4b8f-9921-a235a9c75b69\""), name: Some("\"新人专享\""), pc_link: Some("\"http://115.220.9.139:8002/newuser/\""), h5_link: Some("\"http://115.220.9.139:8002/newuser/\""), remark: Some("\"\""), create_time: Some("\"2019-05-27 10:25:41\""), version: Some(6), delete_flag: Some(1) }, Activity { id: Some("\"dfbdd779-5f70-4b8f-9921-c235a9c75b69\""), name: Some("\"新人专享\""), pc_link: Some("\"http://115.220.9.139:8002/newuser/\""), h5_link: Some("\"http://115.220.9.139:8002/newuser/\""), remark: Some("\"\""), create_time: Some("\"2019-05-27 10:25:41\""), version: Some(6), delete_flag: Some(1) }]

*/
#[test]
fn test_exec_sql(){
    //初始化rbatis
    let mut rbatis = init_rbatis().unwrap();
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


/**

分页查询数据，测试

2020-01-06T16:35:15.969770+08:00 INFO rbatis::core::rbatis - [rbatis] Query ==>  select * from biz_activity where name = '新人专享' AND delete_flag = 1 LIMIT 1,5
2020-01-06T16:35:16.091525+08:00 INFO rbatis::core::rbatis - [rbatis] ReturnRows <== 1
2020-01-06T16:35:16.091848+08:00 INFO rbatis::core::rbatis - [rbatis] Query ==>  select count(1) from biz_activity where name = '新人专享' AND delete_flag = 1
2020-01-06T16:35:16.118317+08:00 INFO rbatis::core::rbatis - [rbatis] ReturnRows <== 1
[rbatis] result==>  IPage { total: 2, size: 5, current: 1, records: Some([Activity { id: Some("\"dfbdd779-5f70-4b8f-9921-c235a9c75b69\""), name: Some("\"新人专享\""), pc_link: Some("\"http://115.220.9.139:8002/newuser/\""), h5_link: Some("\"http://115.220.9.139:8002/newuser/\""), remark: Some("\"\""), create_time: Some("\"2019-05-27 10:25:41\""), version: Some(6), delete_flag: Some(1) }]) }
*/
#[test]
fn test_exec_select_page(){
    //初始化rbatis
    let mut rbatis = init_rbatis().unwrap();
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data:IPage<Activity> = rbatis.select_page("Example_ActivityMapper.xml",  &mut json!({
       "name":"新人专享",
    }), &IPage::new(1,5)).unwrap();
    println!("[rbatis] result==>  {:?}", data);
}

#[test]
fn test_exec_select_page_custom(){
    //初始化rbatis
    let mut rbatis = init_rbatis().unwrap();
    //执行到远程mysql 并且获取结果,Result<serde_json::Value, String>,或者 Result<Activity, String> 等任意类型
    let data:IPage<Activity> = rbatis.select_page_custom("Example_ActivityMapper.xml",  &mut json!({
       "name":"新人专享",
    }), &IPage::new(1,5),"select_by_page").unwrap();
    println!("[rbatis] result==>  {:?}", data);
}