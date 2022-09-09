#[macro_use]
extern crate rbatis;

pub mod model;
use model::*;

use rbatis::executor::Executor;
use rbatis::rbdc::datetime::FastDateTime;

#[html_sql("example/example.html")]
async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    dt: &FastDateTime,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = select_by_condition(&mut rb.clone(), "test", &FastDateTime::now().set_micro(0))
        .await
        .unwrap();
    println!("{:?}", a);
}
