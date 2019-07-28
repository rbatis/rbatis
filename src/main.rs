#![feature(fn_traits)]
#![feature(test)]

extern crate mysql;

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


use hello_macro_derive::HelloMacro;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;


fn main() {
   // Pancakes::hello_macro();
   // String::hello_macro();
    let s=vec!["String".to_string()];
    testf(s);

}

fn testf<T:HelloMacro>(arg:T){
    let name=T::decode_name();
    println!("{}",name);
}