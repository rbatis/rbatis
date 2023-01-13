pub mod model;

use crate::model::{init_db, BizActivity};
use rbatis::executor::{RBatisConnExecutor, RBatisTxExecutor};
use rbatis::rbdc::datetime::FastDateTime;

rbatis::impl_insert!(BizActivity {});
rbatis::impl_delete!(BizActivity {});
#[tokio::main]
pub async fn main() {
    let _ = fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = init_db().await;
    //clear data
    let _ = BizActivity::delete_in_column(&mut rb.clone(),"id",&["2","3","4"]).await;
    tx_run(rb.acquire_begin().await.unwrap(), "2",true).await;
    tx_conn(rb.acquire().await.unwrap()).await;
    tx_tx(rb.acquire_begin().await.unwrap()).await;
    //wait log flush
    log::logger().flush();
}

async fn tx_run(tx: RBatisTxExecutor, id:&str,forget_commit: bool) {
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            println!("rollback");
        }
    });
    let t = BizActivity {
        id: Some(id.into()),
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
    let _ = BizActivity::insert(&mut tx, &t).await;
    //if not commit or rollback,will run 'defer_async' rollback()
    if !forget_commit {
        tx.commit().await.unwrap();
    }
}

async fn tx_conn(executor: RBatisConnExecutor) {
    let tx = executor.begin().await.unwrap();
    tx_run(tx, "3",true).await;
}

async fn tx_tx(mut tx: RBatisTxExecutor) {
    tx.commit().await.unwrap();//finish last tx
    let tx = tx.begin().await.unwrap();
    tx_run(tx, "4",true).await;
}