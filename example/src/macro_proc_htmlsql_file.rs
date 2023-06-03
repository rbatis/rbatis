#[macro_use]
extern crate rbatis;

pub mod init;

use init::*;
use serde_json::json;

use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;

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

#[html_sql("example/example.html")]
async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    dt: &DateTime,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = select_by_condition(&mut rb.clone(), "test", &DateTime::now().set_micro(0))
        .await
        .unwrap();
    println!("{}", json!(a));
}
