pub mod init;

use serde_json::json;
use rbatis::rbdc::datetime::DateTime;
use crate::init::{init_db};
use rbs::to_value;

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    //query
    let table: Option<BizActivity> = rb
        .query_decode("select * from biz_activity limit ?", vec![to_value!(1)])
        .await
        .unwrap();
    //exec
    let result = rb
        .exec("update biz_activity set status = 0 where id > 0", vec![])
        .await
        .unwrap();
    log::logger().flush();
    println!(">>>>> table={}", json!(table));
    println!(">>>>> exec={}", result);
}
