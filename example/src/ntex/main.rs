#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use chrono::NaiveDateTime;
use rbatis::crud::{CRUD};
use rbatis::rbatis::Rbatis;
use ntex::web::{middleware, App, Error, HttpResponse};
use ntex::web;
use std::sync::Arc;
use std::cell::Cell;

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

type DBPool = Arc<Rbatis>;


#[web::get("/")]
async fn index(pool: web::types::Data<DBPool>) -> Result<HttpResponse, Error> {
    let v = pool.fetch_list::<BizActivity>().await.unwrap_or_default();
    Ok(HttpResponse::Ok().json(&v))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    let rbatis = Rbatis::new();
    //link database
    rbatis.link(MYSQL_URL).await.unwrap();

    println!("Starting server at: http://127.0.0.1:8000");
    let rb_clone = DBPool::new(rbatis);
    // Start HTTP server
    web::server(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(rb_clone.clone())
            .wrap(middleware::Logger::default())
            .service((index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}