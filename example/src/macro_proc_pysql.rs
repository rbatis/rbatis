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
use rbatis::Error;
use std::fs::File;
use std::io::Read;

use rbatis::rbatis::Rbatis;
use rbatis::sql::page::{Page, PageRequest};
use rbatis::rbdc::datetime::FastDateTime;
use rbs::{to_value, Value};

use crate::{init_sqlite, BizActivity};

#[py_sql("select * from biz_activity where delete_flag = 0")]
async fn py_ctx_id(rb: &Rbatis) -> Vec<BizActivity> {
    impled!()
}

///select page must have  '?:&PageRequest' arg and return 'Page<?>'
#[py_sql(
    "`select * from biz_activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`"
)]
async fn py_select_page(rb: &mut dyn Executor, name: &str) -> Result<Vec<BizActivity>, Error> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = py_select_page(&mut rb.clone(),  "test")
        .await
        .unwrap();
    println!(">>>>>>>>>>>> {:?}", a);
}
