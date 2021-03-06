#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDateTime;
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
    pub create_time: Option<NaiveDateTime>,
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

// init global rbatis pool
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}


async fn index() -> impl Responder {
    let v = RB.fetch_list::<BizActivity>().await.unwrap();
    HttpResponse::Ok().body(serde_json::json!(v).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //link database
    RB.link(MYSQL_URL).await.unwrap();
    //router
    HttpServer::new(|| {
        App::new()
            // or you can crate on actix-data
            // .data(std::sync::Arc::new({
            //     let rb=Rbatis::new();
            //     rb.link(MYSQL_URL).await.unwrap();
            //     rb
            // }))
            .route("/", web::get().to(index))
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
