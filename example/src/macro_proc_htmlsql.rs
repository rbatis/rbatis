#[macro_use]
extern crate rbatis;
pub mod init;

use serde_json::json;
use init::*;
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;


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

// Clion Smart tips: click code, choose 'Inject Language or Reference', and then choose html
#[html_sql(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">
  <select id="select_by_condition">
        `select * from biz_activity`
        <where>
         <if test="a">
                ` and name like #{name}`
            </if>
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
        </where>
  </select>"#)]
async fn select_by_condition(
    rb: &mut dyn Executor,
    name: &str,
    dt: &DateTime,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_db().await;
    let a = select_by_condition(
        &mut rb.clone(),
        "test",
        &DateTime::now().set_micro(0),
        false,
    )
    .await
    .unwrap();
    println!("{}", json!(a));
}
