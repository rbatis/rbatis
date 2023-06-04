#[macro_use]
extern crate rbatis;

pub mod init;

use crate::init::init_db;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::IntoSql;
use rbs::value::map::ValueMap;
use rbs::Value;
use serde_json::json;

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

impl_select!(BizActivity{select_by_map(logic:ValueMap) -> Option => "`where ${logic.sql()} limit 1`"});

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
    .expect("rbatis init fail");
    let mut rb = init_db().await;
    let mut logic = ValueMap::new();
    logic.insert("id = ".into(), Value::String("221".to_string()));
    logic.insert("and id != ".into(), Value::String("222".to_string()));
    let data = BizActivity::select_by_map(&mut rb, logic).await;
    println!("select_by_method = {}", json!(data));
}
