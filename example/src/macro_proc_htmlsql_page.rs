#[macro_use]
extern crate rbatis;

pub mod model;

use model::*;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;

htmlsql_select_page!(select_page_data(name: &str, dt: &FastDateTime) -> BizActivity => "example/example.html");

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_sqlite().await;
    let a = select_page_data(&mut rb.clone(),
                                          &PageRequest::new(1, 10),
                                          "test",
                                          &FastDateTime::now().set_micro(0))
        .await
        .unwrap();
    println!("{:?}", a);
}
