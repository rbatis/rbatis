#![allow(unused_must_use)]
#[macro_use]
extern crate rbatis;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;

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

async fn index(rb: web::Data<Arc<Rbatis>>) -> impl Responder {
    let v = rb.fetch_list::<BizActivity>().await.unwrap_or_default();
    HttpResponse::Ok()
        .insert_header(("Content-Type", "text/json;charset=UTF-8"))
        .json(v)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //log
    fast_log::init(fast_log::config::Config::new().console());
    //init rbatis . also you can use  pub static RB:Lazy<Rbatis> = Lazy::new(||Rbatis::new()); replace this
    log::info!("linking database...");
    let rb = example::init_sqlite_path("").await;
    let rb = Arc::new(rb);
    log::info!("linking database successful!");

    log::info!("start on http://127.0.0.1:8080");
    //router
    HttpServer::new(move || {
        App::new()
            //add into actix-web data
            .app_data(web::Data::new(rb.to_owned()))
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
