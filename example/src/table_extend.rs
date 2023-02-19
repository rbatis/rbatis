pub mod model;

use crate::model::init_db;
use rbatis::crud;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Base {
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    #[serde(flatten)]
    pub base: Base,
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
crud!(BizActivity {});

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    let datas = BizActivity::select_all(&mut rb.clone()).await.unwrap();
    println!("{}", json!(datas));
}
