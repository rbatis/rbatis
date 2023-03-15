#[macro_use]
extern crate rbatis;

pub mod model;

use serde_json::json;
use rbatis::rbdc::datetime::DateTime;
use crate::model::{init_db, BizActivity};
use rbatis::sql::page::PageRequest;
use rbs::Value;

//crud!(BizActivity {},"biz_activity");//custom table name
//impl_select!(BizActivity{select_all_by_id(table_name:&str,id:&str) => "`where id = #{id}`"}); //custom table name
crud!(BizActivity {});
impl_select!(BizActivity{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(BizActivity{select_by_id(id:&str) -> Option => "`where id = #{id} limit 1`"});
impl_update!(BizActivity{update_by_name(name:&str) => "`where id = '2'`"});
impl_delete!(BizActivity {delete_by_name(name:&str) => "`where name= '2'`"});
impl_select_page!(BizActivity{select_page() =>"
     if !sql.contains('count'):
       `order by create_time desc`"});
impl_select_page!(BizActivity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

// sql() method write in rbatis::sql::methods.rs
use rbatis::sql::IntoSql;
use rbs::value::map::ValueMap;
impl_select!(BizActivity{select_by_method(ids:&[&str],logic:ValueMap) -> Option => "`where ${logic.sql()} and id in ${ids.sql()}   limit 1`"});

#[tokio::main]
pub async fn main() {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )
    .expect("rbatis init fail");
    let mut rb = init_db().await;
    let t = BizActivity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("2".to_string()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let tables = [t.clone(), {
        let mut t3 = t.clone();
        t3.id = "3".to_string().into();
        t3
    }];

    let data = BizActivity::insert(&mut rb, &t).await;
    println!("insert = {}", json!(data));

    let _data = BizActivity::delete_by_name(&mut rb, "2").await;
    let _data = BizActivity::delete_by_name(&mut rb, "3").await;

    let data = BizActivity::insert_batch(&mut rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));

    let data = BizActivity::update_by_column_batch(&mut rb, &tables, "id").await;
    println!("update_by_column_batch = {}", json!(data));

    let data = BizActivity::select_all_by_id(&mut rb, "1", "1").await;
    println!("select_all_by_id = {}", json!(data));

    let data = BizActivity::select_by_id(&mut rb, "1").await;
    println!("select_by_id = {}", json!(data));

    let data = BizActivity::update_by_column(&mut rb, &t, "id").await;
    println!("update_by_column = {}", json!(data));

    let data = BizActivity::update_by_name(&mut rb, &t, "test").await;
    println!("update_by_name = {}", json!(data));

    let data = BizActivity::select_page(&mut rb, &PageRequest::new(1, 10)).await;
    println!("select_page = {}", json!(data));

    let data = BizActivity::select_page_by_name(&mut rb, &PageRequest::new(1, 10), "").await;
    println!("select_page_by_name = {}", json!(data));

    let data = BizActivity::delete_by_column(&mut rb, "id", "2").await;
    println!("delete_by_column = {}", json!(data));

    let data = BizActivity::delete_by_name(&mut rb, "2").await;
    println!("delete_by_column = {}", json!(data));

    let data = BizActivity::select_in_column(&mut rb, "id", &["1", "2", "3"]).await;
    println!("select_in_column = {}", json!(data));

    let mut logic = ValueMap::new();
    logic.insert("id = ".into(), Value::I32(1));
    logic.insert("and id != ".into(), Value::I32(2));
    let data = BizActivity::select_by_method(&mut rb, &["1", "2"], logic).await;
    println!("select_by_method = {}", json!(data));

    let data = BizActivity::delete_in_column(&mut rb, "id", &["1", "2", "3"]).await;
    println!("delete_in_column = {}", json!(data));
}
