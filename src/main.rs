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