#[macro_use]
extern crate rbatis;
pub mod model;

use model::*;
use rbatis::executor::Executor;
use rbatis::rbdc::datetime::FastDateTime;

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
    dt: &FastDateTime,
    a: bool,
) -> rbatis::Result<Vec<BizActivity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    //use static ref
    let rb = init_sqlite().await;
    let a = select_by_condition(
        &mut rb.clone(),
        "test",
        &FastDateTime::now().set_micro(0),
        false,
    )
    .await
    .unwrap();
    println!("{:?}", a);
}
