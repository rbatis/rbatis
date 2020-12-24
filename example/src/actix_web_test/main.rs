#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
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

async fn index() -> impl Responder {
    let v = RB.list::<BizActivity>("").await.unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&v).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //日志追加器
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //ORM
    RB.link(MYSQL_URL).await.unwrap();
    //路由
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
