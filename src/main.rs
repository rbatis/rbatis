#![feature(test)]
#![feature(fn_traits)]
extern crate test;

extern crate mysql;

pub mod example;
pub mod ast;
pub mod utils;

use test::Bencher;

pub mod engines;
pub mod core;

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
use engines::RustExpressionEngine;


use rbatis_macro_derive::RbatisMacro;

use rbatis_macro::RbatisMacro;

#[derive(RbatisMacro)]
struct Pancakes;


fn main() {
   // Pancakes::rbatis_macro();
   // String::rbatis_macro();
    let s=vec!["String".to_string()];
    testf(s);

}

fn testf<T:RbatisMacro>(arg:T){
    let name=T::decode_name();
    println!("{}",name);
}

#[bench]
fn Bench_Test(b: &mut Bencher) {

    b.iter( || {

        // v.push(2);
    });
}