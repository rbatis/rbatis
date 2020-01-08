#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
extern crate serde_yaml;
extern crate rdbc;
extern crate rdbc_mysql;
extern crate rdbc_postgres;


use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

use async_std::future;
use async_std::task;
use chrono::Local;
use log::{error, info, warn};
use log4rs;
use serde_json::json;
use uuid::Uuid;
use xml::EventReader;
use xml::reader::XmlEvent;

use example::activity::Activity;
use utils::bencher::Bencher;
use utils::time_util;

use crate::security::arg_filter::ArgFilter;
pub mod example;
pub mod ast;
pub mod utils;
pub mod crud;
pub mod security;
pub mod convert;
pub mod server;
pub mod engine;
pub mod core;
pub mod decode;
pub mod tx;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

#[async_std::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("=====================================================================================");
    error!("================================= [rbatis] now is started============================");
    warn!("=====================================================================================");
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
         let js:serde_json::Value=serde_json::from_str(r#"{"id":"","name":"","version":0}"#).unwrap();
//        Uuid::new_v4();
    });
}