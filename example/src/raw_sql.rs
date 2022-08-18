pub mod model;

use crate::model::{init_sqlite, BizActivity};
use fast_log::sleep;
use rbs::to_value;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_sqlite().await;
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
