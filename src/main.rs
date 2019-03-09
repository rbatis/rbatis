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
use std::fs;

fn main() {
    let node_string = NodeString { buf: "".to_string() };
    let mut m = HashMap::new();
    m.insert("v", String::from("dsa"));
    let s = node_string.Eval(&m);
    println!("{}", s);
    println!("{}", m["v"].as_str());
    TimeUtil::Count_Time(1, Local::now());
}


#[test]
fn Test_load() {
    let path = fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap();
    println!("Name: {}", path)
}

//load a xml file
#[test]
fn Test_load_file() {
    // --snip--
    let filename = "./src/example/Example_ActivityMapper.xml";
    println!("In file {}", filename);
    let content = fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap();
    println!("With text:/n{}", content);
}

//load xml
#[test]
fn Test_load_xml() {
    println!(">>>>>>>>>>>>>>>>>>>>>>start load >>>>>>>>>>>>>>>>>>>>>>>");
    let content = fs::read_to_string("./src/example/Example_ActivityMapper.xml").unwrap();
    utils::XmlLoader::LoadXml(content);
}
