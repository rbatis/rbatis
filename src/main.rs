#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
extern crate rbatis_macro_derive;

pub mod example;
pub mod ast;
pub mod utils;
pub mod crud;
pub mod security;

//use test::Bencher;

pub mod engine;
pub mod core;
pub mod decode;

use utils::time_util;
use chrono::Local;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use std::fs;
use serde_json::json;
use rbatis_macro_derive::RbatisMacro;
use rbatis_macro::RbatisMacro;
use std::thread::{sleep, spawn};
use std::time::Duration;
use async_std::task;

use std::sync::Mutex;
use utils::bencher::Bencher;
use example::activity::Activity;
use async_std::future;
use crate::security::arg_filter::ArgFilter;
use uuid::Uuid;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}


#[async_std::main]
async fn main() {
//    ARRAY.lock().unwrap().push(1);
//    println!("{:?}",ARRAY.lock().unwrap().get(0).unwrap());
    let id = task::current().id();
    println!("{:?}", id);
    let task = task::spawn(async {
        let id = task::current().id();
        println!("{:?}", id);
        task::sleep(Duration::from_millis(1000)).await;
    });
    println!("waiting for the task");
    let res = task.await;
    println!("task ended with result {:?}", res);
}




//cargo test --release --package rbatis --bin rbatis bench_main --all-features -- --nocapture --exact
#[test]
fn bench_main() {
    let mut b =Bencher::new(1000000);
    b.iter( || {
         //println!("asdf");
//         let  mut js:serde_json::Value=serde_json::from_str(r#"{"id":"","name":"","version":0}"#).unwrap();
//         arg_filter::filter(&mut js);
        Uuid::new_v4();
    });
}