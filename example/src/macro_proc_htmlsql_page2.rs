#[macro_use]
extern crate rbatis;

pub mod model;

use model::*;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde_json::json;

htmlsql_select_page!(select_page_data2(name: &str, dt: &DateTime) -> BizActivity => r#"<select id="select_page_data">
        `select `
        <if test="do_count == true">
            `count(1)`
        </if>
        <if test="do_count == false">
            `*`
        </if>
        ` from biz_activity`
        <where>
            <if test="name != ''">
                ` and name like #{name}`
            </if>
            <if test="dt >= '2009-12-12 00:00:00'">
                ` and create_time < #{dt}`
            </if>
            <choose>
                <when test="true">
                    ` and id != '-1'`
                </when>
                <otherwise>and id != -2</otherwise>
            </choose>
            ` and `
            <trim prefixOverrides=" and">
                ` and name != '' `
            </trim>
            <if test="do_count == false">
                ` limit ${page_no},${page_size}`
            </if>
        </where>
    </select>"#);

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = select_page_data2(
        &mut rb.clone(),
        &PageRequest::new(1, 10),
        "test",
        &DateTime::now().set_micro(0),
    )
    .await
    .unwrap();
    println!("{}", json!(a));
}
