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

use std::fs::File;
use std::io::Read;
use rbatis::executor::{ExecutorMut, RbatisExecutor};
use rbatis::sql::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;


///select page must have  '?:&PageRequest' arg and return 'Page<?>'
#[html_sql("example/example.html")]
async fn select_by_condition(
    mut rb: RbatisExecutor<'_>,
    page_req: &PageRequest,
    name: &str,
    dt: &rbatis::core::datetime::FastDateTime,
) -> Vec<BizActivity> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = select_by_condition(
        rb.as_executor(),
        &PageRequest::new(1, 10),
        "test",
        &rbatis::core::datetime::FastDateTime::now().set_micro(0),
    )
        .await
        .unwrap();
    println!("{:?}", a);
}

