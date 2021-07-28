#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
use example::BizActivity;
use rbatis::crud::{CRUD};
use rbatis::rbatis::Rbatis;
use std::sync::Arc;
use rocket::fairing::AdHoc;
use rocket::{Rocket, State, Build, futures};
//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";
#[get("/")]
async fn hello(rb:&State<Arc<Rbatis>>) -> String {
    let v = rb.fetch_list::<BizActivity>().await.unwrap();
    serde_json::json!(v).to_string()
}
#[rocket::main]
async fn main() {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //link database
    let rb=Rbatis::new();
    rb.link(MYSQL_URL).await.unwrap();
    let rb=Arc::new(rb);
    rocket::build()
        .mount("/", routes![hello])
        .attach(AdHoc::on_ignite("Rbatis Database",|rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await;
}