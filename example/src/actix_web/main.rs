#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use std::sync::Arc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use rbatis::crud::{CRUD};
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

impl Default for BizActivity {
    fn default() -> Self {
        BizActivity {
            id: None,
            name: None,
            pc_link: None,
            h5_link: None,
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: None,
            remark: None,
            create_time: None,
            version: None,
            delete_flag: None,
        }
    }
}

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

async fn index(rb: web::Data<Arc<Rbatis>>) -> impl Responder {
    let v = rb.fetch_list::<BizActivity>().await.unwrap();
    HttpResponse::Ok().set_header("Content-Type","text/json;charset=UTF-8").body(serde_json::json!(v).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //log
    fast_log::init_log("requests.log", log::Level::Info, None, true);
    //init rbatis . also you can use  lazy_static! { static ref RB: Rbatis = Rbatis::new(); } replace this
    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");
    //router
    HttpServer::new(move || {
        App::new()
            //add into actix-web data
            .data(rb.to_owned())
            .route("/", web::get().to(index))
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
