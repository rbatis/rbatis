pub mod model;
use crate::model::{init_sqlite, BizActivity};
use rbatis::executor::Executor;
use rbdc::datetime::FastDateTime;

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console());
    let mut rb = init_sqlite().await;
    let mut t = BizActivity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: None,
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(FastDateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let mut tx = rb.acquire_begin().await.unwrap();
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            println!("rollback");
        }
    });
    tx.exec("select 1", vec![]).await.unwrap();
    println!("yes forget commit");
}
