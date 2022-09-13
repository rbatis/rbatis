#[macro_use]
extern crate rbatis;

pub mod model;

use crate::{init_db, BizActivity};
use model::*;
use rbatis::executor::Executor;
use rbatis::Error;

#[py_sql(
    "`select * from biz_activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`"
)]
async fn py_select(rb: &mut dyn Executor, name: &str) -> Result<Vec<BizActivity>, Error> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = py_select(&mut rb.clone(), "").await.unwrap();
    println!(">>>>>>>>>>>> {:?}", a);
}
