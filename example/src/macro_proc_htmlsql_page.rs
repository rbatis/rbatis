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
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;

htmlsql_select_page!(BizActivity{select_page_data(name: &str, dt: &FastDateTime) => "example/example.html"});

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_sqlite().await;
    let a = BizActivity::select_page_data(&mut rb.clone(),
                                          &PageRequest::new(1, 10),
                                          "test",
                                          &FastDateTime::now().set_micro(0))
        .await
        .unwrap();
    println!("{:?}", a);
}
