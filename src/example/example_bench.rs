use std::fs;
use crate::rbatis::Rbatis;
use serde_json::{json, Value};
use crate::ast::xml::bind_node::BindNode;
use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
//use test::Bencher;
use chrono::Local;
use crate::utils;
use crate::ast::xml::node_type::NodeType;
use crate::utils::bencher::Bencher;

//lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
//}

//#[async_std::main]
//async fn main() {
//    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
//    info!("=====================================================================================");
//    error!("================================= [rbatis] now is started============================");
//    warn!("=====================================================================================");
////    ARRAY.lock().unwrap().push(1);
////    println!("{:?}",ARRAY.lock().unwrap().get(0).unwrap());
//    let id = task::current().id();
//    println!("{:?}", id);
//    let task = task::spawn(async {
//        let id = task::current().id();
//        println!("{:?}", id);
//        task::sleep(Duration::from_millis(1000)).await;
//    });
//    println!("waiting for the task");
//    let res = task.await;
//    println!("task ended with result {:?}", res);
//}


#[test]
fn bench_main() {
    let mut b =Bencher::new(1000000);
    b.iter( || {
        //println!("asdf");
        let js:serde_json::Value=serde_json::from_str(r#"{"id":"","name":"","version":0}"#).unwrap();
//        Uuid::new_v4();
    });
}


///use Time: 0.202 s,each:2020 nano/op
///use TPS: 495049.50495049503 TPS/s
//cargo test --release --color=always --package rbatis --bin rbatis example::example_bench::bench_main --all-features -- --nocapture --exact
#[test]
fn bench_example() {
    let mut rbatis=Rbatis::new();
    rbatis.set_enable_log(false);//禁用日志以准确获取性能数据
    rbatis.load_xml("Example_ActivityMapper.xml".to_string(),fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap());//加载xml数据
    rbatis.print();//打印已读取的内容

    let data= rbatis.mapper_map.get_mut("Example_ActivityMapper.xml").unwrap().get_mut("select_by_condition").unwrap();
    let arg=&mut json!({
       "name":null,
       "startTime":null,
       "endTime":null,
       "page":null,
       "size":null,
    });

    let mut arg_array=vec![];

    data.eval(arg,&mut rbatis.holder,&mut arg_array);

    let total=100000;
    let now=Local::now();
    for _ in 0..total{
        data.eval(arg,&mut rbatis.holder,&mut arg_array);
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}
