#[macro_use]
extern crate rbatis;

pub mod init;

use serde_json::json;
use init::*;

use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbs::Value;

use crate::{init_db};

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


pub trait IsTest {
    fn is_test(&self) -> bool;
}

impl IsTest for rbs::Value {
    fn is_test(&self) -> bool {
        match self {
            Value::String(v) => v.eq("test"),
            _ => false,
        }
    }
}

/// you can see custom fn on xml:
/// ```xml
/// <if test="name.is_test()">
///    ....
///  </if>
/// ```
#[html_sql(r#"
    <select id="custom_func">
        `select * from biz_activity`
        <where>
            <if test="name.is_test()">
                `and name like #{name}`
            </if>
        </where>
    </select>"#)]
async fn custom_func(rb: &mut dyn Executor, name: &str) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = custom_func(&mut rb.clone(), "test").await.unwrap();
    println!("{}", json!(a));
}
