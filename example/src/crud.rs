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

use crate::model::{init_sqlite, BizActivity};
use fast_log::sleep;
use rbatis::{
    executor::Executor,
    sql::page::{Page, PageRequest},
};
use rbdc::datetime::FastDateTime;
use std::time::Duration;

impl_insert!(BizActivity {});
impl_select!(BizActivity {});
impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => " where id = #{id} and name = #{name}"});
impl_select!(BizActivity{select_by_id(id:String) -> Option => " where id = #{id} limit 1"});
impl_update!(BizActivity {});
impl_update!(BizActivity{update_by_name(name:&str)}=> " where id = 1");
impl_delete!(BizActivity {});
impl_delete!(BizActivity {delete_by_name(name:&str)}=> " where name= '2'");
impl_select_page!(BizActivity{select_page(name:&str) => " where name != #{name}"});


#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
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
    let af = BizActivity::insert(&mut rb, &t).await;
    println!("insert = {:?}", af);

    sleep(Duration::from_secs(2));

    let data = BizActivity::select_all_by_id(&mut rb, "1", "1").await;
    println!("select_all_by_id = {:?}", data);

    sleep(Duration::from_secs(2));

    let data = BizActivity::select_by_id(&mut rb, "1".to_string()).await;
    println!("select_by_id = {:?}", data);

    sleep(Duration::from_secs(2));
    let data = BizActivity::update_by_column(&mut rb, &t, "id").await;
    println!("update_by_column = {:?}", data);

    sleep(Duration::from_secs(2));
    let data = BizActivity::update_by_name(&mut rb, &t, "test").await;
    println!("update_by_name = {:?}", data);

    sleep(Duration::from_secs(2));
    let data = BizActivity::delete_by_column(&mut rb, "id", &"2".into()).await;
    println!("delete_by_column = {:?}", data);

    sleep(Duration::from_secs(2));
    let data = BizActivity::delete_by_name(&mut rb, "2").await;
    println!("delete_by_column = {:?}", data);

    sleep(Duration::from_secs(2));
    let data = BizActivity::select_page(&mut rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page = {:?}", data);
}
