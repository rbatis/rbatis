#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
use example::BizActivity;
use rbatis::crud::{CRUD};
use rbatis::rbatis::Rbatis;
//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";
// init global rbatis pool
lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
}
#[get("/")]
async fn hello() -> String {
    let v = RB.fetch_list::<BizActivity>().await.unwrap();
    serde_json::json!(v).to_string()
}
#[rocket::main]
async fn main() {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    //link database
    RB.link(MYSQL_URL).await.unwrap();

    rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await;
}