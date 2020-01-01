use std::fs;
use crate::core::rbatis::Rbatis;
use serde_json::{json, Value};
use crate::ast::xml::bind_node::BindNode;
use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
//use test::Bencher;
use chrono::Local;
use crate::utils;
use crate::ast::xml::node_type::NodeType;
use crate::utils::bencher::Bencher;


///fn test_benchmark_tps()
///use Time: 0.042 s,each:420 nano/op
///use TPS: 2380952.3809523806 TPS/s
///



///use Time: 0.202 s,each:2020 nano/op
///use TPS: 495049.50495049503 TPS/s
//cargo.exe test --release --color=always --package rbatis --bin rbatis example::example_bench::bench_main --all-features -- --nocapture --exact
#[test]
fn bench_main() {

    let mut rbatis=Rbatis::new();
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


    data.eval(arg,&mut rbatis.holder);

    let total=100000;
    let now=Local::now();
    for _ in 0..total{
        data.eval(arg,&mut rbatis.holder);
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}
