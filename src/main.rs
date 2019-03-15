mod ast;
mod utils;
mod engines;
mod lib;

use self::utils::time_util;
use self::ast::NodeString::NodeString;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use ast::Node::Node;
use std::fs::File;
use std::io::{Read, BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use std::fs;

fn main() {
    utils::print_util::print_rust_mybatis();
    let now=Local::now();



    time_util::count_time(1, now);
}
