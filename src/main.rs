#![feature(fn_traits)]
#![feature(test)]

extern crate r2d2;
extern crate r2d2_mysql;
mod example;
mod ast;
mod utils;
extern crate test;
use test::Bencher;
mod engines;
mod lib;
mod SqlBuilder;

use self::utils::time_util;
//use utils::TimeUtil;
use chrono::Local;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use std::fs;
use serde_json::json;
use lib::RustExpressionEngine;

//fn main() {
//    utils::print_util::print_rust_mybatis();
//    let now=Local::now();
//
//
//
//    time_util::count_time(1, now);
//}

macro_rules! foo {
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

fn main() {
    foo!(y => 3);
}