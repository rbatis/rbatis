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
use rbatis::rbatis::Rbatis;
use rbatis::sql::page::{Page, PageRequest};
use std::fs::File;
use std::io::Read;

use crate::{init_sqlite, BizActivity};

/// doc you can see https://rbatis.github.io/rbatis.io/#/en/
#[sql("select * from biz_activity where delete_flag = ?")]
async fn raw_sql(rb: &Rbatis, delete_flag: &i32) -> Vec<BizActivity> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = raw_sql(&rb, &0).await.unwrap();
    println!("{:?}", a);
}
