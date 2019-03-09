mod ast;
mod utils;

use self::utils::TimeUtil;
use self::ast::NodeString::NodeString;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use crate::ast::Node::Node;
use std::fs::File;
use std::io::{Read, BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;

//fn main() {
//    let node_string = NodeString { buf: "".to_string() };
//
//    let mut m = HashMap::new();
//    m.insert("v",String::from("dsa"));
//
//
//    let s = node_string.Eval(&m);
//    println!("{}", s);
//
//    println!("{}", m["v"].as_str());
//
//
//    TimeUtil::Count_Time(1, Local::now());
//}

////load a xml file
//fn main() {
//    // --snip--
//    let filename="D:/RustProject/RustMybatis/src/example/Example_ActivityMapper.xml";
//    println!("In file {}", filename);
//
//    let mut f = File::open(filename).expect("file not found");
//
//    let mut contents= String::new();
//    f.read_to_string(&mut contents).expect("something went wrong reading the file");
//
//    println!("With text:/n{}", contents);
//}

//load xml
#[test]
fn Test(){
    println!(">>>>>>>>>>>>>>>>>>>>>>start load >>>>>>>>>>>>>>>>>>>>>>>");
    utils::XmlLoader::LoadXml(String::from("D:/RustProject/RustMybatis/src/example/Example_ActivityMapper.xml"));
}
