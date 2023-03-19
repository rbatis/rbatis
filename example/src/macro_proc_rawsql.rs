#[macro_use]
extern crate rbatis;

pub mod model;

use crate::{init_db, BizActivity};
use model::*;
use rbatis::rbatis::Rbatis;
use serde_json::json;

/// doc you can see https://rbatis.github.io/rbatis.io
#[sql("select * from biz_activity where delete_flag = ?")]
async fn raw_sql(rb: &Rbatis, delete_flag: &i32) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = raw_sql(&rb, &0).await.unwrap();
    println!("{}", json!(a));
}
