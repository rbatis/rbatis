pub mod model;

use std::time::Duration;
use fast_log::sleep;
use rbdc::datetime::FastDateTime;
use crate::model::{init_sqlite, BizActivity};


#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    let mut rb = init_sqlite().await;
    let table:Option<BizActivity>=rb.fetch_decode("select * from biz_activity limit 1",vec![]).await.unwrap();
    let count:u64=rb.fetch_decode("select count(1) as count from biz_activity",vec![]).await.unwrap();
    sleep(Duration::from_secs(1));
    println!(">>>>> table={:?}",table);
    println!(">>>>> count={}",count);
}