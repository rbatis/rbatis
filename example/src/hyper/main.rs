#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;


use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rbatis::crud::{CRUDMut, CRUD};
use rbatis::rbatis::Rbatis;
use std::convert::Infallible;
use once_cell::sync::Lazy;

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

pub static RB:Lazy<Rbatis> = Lazy::new(||Rbatis::new());

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let v = RB.fetch_list::<BizActivity>().await.unwrap_or_default();
    Ok(Response::new(Body::from(serde_json::json!(v).to_string())))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    fast_log::init_log("requests.log", log::Level::Info, None, true);
    log::info!("linking database...");
    RB.link(MYSQL_URL).await.expect("rbatis link database fail");
    log::info!("linking database successful!");
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });
    let addr = ([127, 0, 0, 1], 8000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
