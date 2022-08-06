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
use rbatis::sql::page::{Page, PageRequest};
use std::fs::File;
use std::io::Read;
use rbdc::datetime::FastDateTime;

///select page must have  '?:&PageRequest' arg and return 'Page<?>'
#[html_sql("example/example.html")]
async fn select_by_condition(
    rb: &mut dyn Executor,
    page_req: &PageRequest,
    name: &str,
    dt: &FastDateTime,
) -> Vec<BizActivity> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = select_by_condition(
        &mut rb.clone(),
        &PageRequest::new(1, 10),
        "test",
        &FastDateTime::now().set_micro(0),
    )
    .await
    .unwrap();
    println!("{:?}", a);
}
