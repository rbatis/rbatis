#[macro_use]
extern crate rbatis;

pub mod init;

use crate::init_db;
use crate::rbatis::sql::IntoSql;
use init::*;
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::Error;
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

#[py_sql(
    "`select * from biz_activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}"
)]
async fn py_select(
    rb: &mut dyn Executor,
    name: &str,
    ids: &[i32],
) -> Result<Vec<BizActivity>, Error> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    let l = fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = py_select(&mut rb.clone(), "", &[1, 2, 3]).await.unwrap();
    println!(">>>>>>>>>>>> {}", json!(a));
    l.wait();
}
