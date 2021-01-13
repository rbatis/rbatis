#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use warp::Filter;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

#[crud_enable]
#[derive(Clone, Debug)]
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
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

//示例 mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  static ref RB:Rbatis=Rbatis::new();
}

#[tokio::main]
async fn main() {
    //日志追加器
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //ORM
    RB.link(MYSQL_URL).await.unwrap();

    let routes = warp::get()
        .and(warp::query::query())
        // and_then create a `Future` that will simply wait N seconds...
        .and_then(sleepy);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn sleepy(arg:HashMap<String,serde_json::Value>) -> Result<impl warp::Reply, Infallible> {
    let v = RB.list::<BizActivity>("").await.unwrap();
    Ok(format!("{}", serde_json::json!(v)))
}


