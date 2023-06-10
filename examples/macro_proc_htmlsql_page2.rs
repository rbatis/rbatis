#[macro_use]
extern crate rbatis;

pub mod init;

use init::*;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::PageRequest;
use serde_json::json;

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

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
