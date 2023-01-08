pub mod model;
use crate::model::init_db;
#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    //set pool max size
    rb.get_pool().unwrap().resize(100);
    println!(">>>>> state={:?}",  rb.get_pool().unwrap().status());
    rb.get_pool().unwrap().close();
}
