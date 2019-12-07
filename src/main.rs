#![feature(test)]
extern crate test;

pub mod example;
pub mod ast;
pub mod utils;
pub mod templete;

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
use engines::RbatisEngine;
use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use std::thread::sleep;
use std::time::Duration;
use async_std::task;


fn main() {
    let task = task::spawn(async {
        task::sleep(Duration::from_millis(1000)).await;
        println!("done");
        "hello"
    });

    task::block_on(async {
        println!("waiting for the task");
        let res = task.await;
        println!("task ended with result {:?}", res);
    });
}


#[bench]
fn bench_main(b: &mut Bencher) {
    b.iter( || {
        // v.push(2);
    });
}