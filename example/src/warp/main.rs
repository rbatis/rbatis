#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;


use rbatis::crud::{CRUDMut, CRUD};
use rbatis::rbatis::Rbatis;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use std::time::Duration;
use warp::Filter;

#[crud_table]
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
    pub create_time: Option<rbatis::DateTimeNative>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}

#[tokio::main]
async fn main() {
    //LOG
    fast_log::init_log("requests.log", log::Level::Info, None, true);

    log::info!("linking database...");
    //ORM
    RB.link(MYSQL_URL).await.expect("rbatis link database fail");
    log::info!("linking database successful!");
    let routes = warp::get()
        .and(warp::query::query())
        // and_then create a `Future` that will simply wait N seconds...
        .and_then(handler);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn handler(arg: HashMap<String, serde_json::Value>) -> Result<impl warp::Reply, Infallible> {
    let v = RB.fetch_list::<BizActivity>().await.unwrap();
    Ok(format!("{}", serde_json::json!(v)))
}
