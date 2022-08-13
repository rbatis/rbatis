pub mod model;

use std::time::Duration;
use fast_log::sleep;
use crate::model::{init_sqlite, BizActivity};
use rbatis::impl_insert;
use rbdc::datetime::FastDateTime;

impl_insert!(BizActivity {});

#[tokio::main]
pub async fn main() {
    let _=fast_log::init(fast_log::Config::new().console());
    let rb = init_sqlite().await;
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
    BizActivity::insert(&mut tx,&t).await.unwrap();
    println!("yes forget commit");
    drop(tx);
    //call sleep make sure tokio runtime not exit!
    sleep(Duration::from_secs(1));
}