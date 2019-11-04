#![feature(async_await)]
#![feature(test)]
extern crate test;
use futures::executor::block_on;

pub mod example;
pub mod ast;
pub mod utils;

use test::Bencher;

pub mod engines;
pub mod core;
pub mod decode;

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
use std::thread::sleep;
use std::time::Duration;

fn main(){
    block_on(fff());
    sleep(Duration::from_secs(3));
}

pub  async fn fff(){
   println!("func!");
}


#[bench]
fn Bench_main(b: &mut Bencher) {
    b.iter( || {
        // v.push(2);
    });
}