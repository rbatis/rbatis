#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use once_cell::sync::Lazy;
use salvo::prelude::*;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;

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

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());

#[fn_handler]
async fn hello(res: &mut Response) {
    let v = RB.fetch_list::<BizActivity>().await.unwrap_or_default();
    res.render_json(&v)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fast_log::init(fast_log::config::Config::new().console());
    log::info!("linking database...");
    RB.link(MYSQL_URL).await.expect("rbatis link database fail");
    log::info!("linking database successful!");
    let addr = "127.0.0.1:8000";
    let server = Server::new(TcpListener::bind(addr)).serve(Router::new().handle(hello));
    println!("Listening on http://{}", addr);
    server.await;
    Ok(())
}
