#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use chrono::NaiveDateTime;
use rbatis::crud::{CRUDMut, CRUD};
use rbatis::rbatis::Rbatis;
use serde_json::{json, Value};
use tide::Request;

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

#[async_std::main]
async fn main() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    log::info!("linking database...");
    RB.link(MYSQL_URL).await.expect("rbatis link database fail");
    log::info!("linking database successful!");
    let mut app = tide::new();
    app.at("/").get(|_: Request<()>| async move {
        let v = RB.fetch_list::<BizActivity>().await;
        Ok(serde_json::json!(v).to_string())
    });
    let addr = "127.0.0.1:8000";
    println!("http server listen on http://{}", addr);
    app.listen(addr).await.unwrap();
}
