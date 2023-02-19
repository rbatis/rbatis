#[macro_use]
extern crate rbatis;

pub mod model;

use serde_json::json;
use model::*;

use rbatis::executor::Executor;
use rbs::Value;

use crate::{init_db, BizActivity};

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
