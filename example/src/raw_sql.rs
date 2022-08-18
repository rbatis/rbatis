#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

pub mod model;

use crate::model::{init_sqlite, BizActivity};
use fast_log::sleep;
use rbatis::rbdc::datetime::FastDateTime;
use rbs::to_value;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    let mut rb = init_sqlite().await;
    let table: Option<BizActivity> = rb
        .fetch_decode("select * from biz_activity limit ?", vec![to_value!(1)])
        .await
        .unwrap();
    let count: u64 = rb
        .fetch_decode("select count(1) as count from biz_activity", vec![])
        .await
        .unwrap();
    sleep(Duration::from_secs(1));
    println!(">>>>> table={:?}", table);
    println!(">>>>> count={}", count);
}
