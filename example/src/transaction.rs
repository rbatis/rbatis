pub mod model;

use crate::model::{init_db, BizActivity};
use rbatis::impl_insert;
use rbatis::rbdc::datetime::FastDateTime;
use std::time::Duration;

impl_insert!(BizActivity {});

#[tokio::main]
pub async fn main() {
    let _ = fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    let t = BizActivity {
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
    let tx = rb.acquire_begin().await.unwrap();
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            println!("rollback");
        }
    });
    //tx.exec("select 1", vec![]).await.unwrap();
    BizActivity::insert(&mut tx, &t).await.unwrap();
    println!("yes forget commit");
    drop(tx);
    //wait log flush
    log::logger().flush();
}
