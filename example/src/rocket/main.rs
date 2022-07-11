#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;

use example::BizActivity;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rocket::fairing::AdHoc;
use rocket::{futures, Build, Rocket, State};
use std::sync::Arc;

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

#[get("/")]
async fn hello(rb: &State<Arc<Rbatis>>) -> String {
    let v = rb.fetch_list::<BizActivity>().await.unwrap_or_default();
    serde_json::json!(v).to_string()
}

#[rocket::main]
async fn main() {
    //log
    fast_log::init(fast_log::config::Config::new().console());
    //link database,also you can use  pub static RB:Lazy<Rbatis> = Lazy::new(||Rbatis::new()); replace this
    log::info!("linking database...");
    let rb = example::init_sqlite_path("").await;
    let rb = Arc::new(rb);
    log::info!("linking database successful!");
    rocket::build()
        .mount("/", routes![hello])
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await;
}
