#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;
pub mod model;

use std::time::Duration;
use fast_log::sleep;
use rbdc::datetime::FastDateTime;
use crate::model::{BizActivity, init_sqlite};

impl_insert!(BizActivity);
impl_select_all!(BizActivity,select_all,"select * from biz_activity where id = #{id}",id:String);
impl_select_one!(BizActivity,find_by_id,"select * from biz_activity where id = #{id} limit 1",id:String);

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    let rb = init_sqlite().await;
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
        delete_flag: Some(1)
    };
    let af= BizActivity::insert(rb.as_executor(),&t).await;
    println!("{:?}",af);

    sleep(Duration::from_secs(2));

    let data=BizActivity::select_all(rb.as_executor(),"1".to_string()).await;
    println!("{:?}",data);

    sleep(Duration::from_secs(2));

    let data=BizActivity::find_by_id(rb.as_executor(),"1".to_string()).await;
    println!("{:?}",data);
}

