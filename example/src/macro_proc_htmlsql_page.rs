#[macro_use]
extern crate rbatis;

pub mod init;

use serde_json::json;
use init::*;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

htmlsql_select_page!(select_page_data(name: &str, dt: &DateTime) -> BizActivity => "example/example.html");

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = select_page_data(
        &mut rb.clone(),
        &PageRequest::new(1, 10),
        "test",
        &DateTime::now().set_micro(0),
    )
    .await
    .unwrap();
    println!("{}", json!(a));
}
