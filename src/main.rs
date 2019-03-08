mod ast;
mod utils;

use utils::TimeUtil;
use ast::NodeString::NodeString;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use crate::ast::Node::Node;

fn main() {
    let node_string = NodeString { buf: "".to_string() };

    let m = &HashMap::new();

    let s = node_string.Eval(m);
    println!("{}", s);


    let v = m.get("v");


    let re = &String::from("s");
    let asf = v.unwrap_or(re);
    println!("{}", asf);


    TimeUtil::Count_Time(1, Local::now());
}