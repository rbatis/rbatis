#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;

pub mod model;
use model::*;

use rbatis::executor::Executor;
use rbatis::rbatis::Rbatis;
use rbs::Value;
use std::fs::File;
use std::io::Read;

use crate::{init_sqlite, BizActivity};

pub trait IsTest {
    fn is_test(&self) -> bool;
}

impl IsTest for rbs::Value {
    fn is_test(&self) -> bool {
        match self {
            Value::String(v) => v.eq("test"),
            _ => false,
        }
    }
}

/// you can see custom fn on xml:
/// ```xml
/// <if test="name.is_test()">
///    ....
///  </if>
/// ```
#[html_sql("example/example.html")]
async fn custom_func(rb: &mut dyn Executor, name: &str) -> Vec<BizActivity> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = custom_func(&mut rb.clone(), "test").await.unwrap();
    println!("{:?}", a);
}
