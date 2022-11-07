pub mod model;

use crate::model::{init_db, BizActivity};
use rbs::to_value;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    //fetch
    let table: Option<BizActivity> = rb
        .fetch_decode("select * from biz_activity limit ?", vec![to_value!(1)])
        .await
        .unwrap();
    //exec
    let result = rb
        .exec("update biz_activity set status = 0 where id > 0", vec![])
        .await
        .unwrap();
    sleep(Duration::from_secs(1)).await;
    println!(">>>>> table={:?}", table);
    println!(">>>>> exec={}", result);
}
