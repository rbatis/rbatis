#![allow(unused_must_use)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use chrono::NaiveDateTime;
use serde_json::{json, Value};
use tide::Request;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;


///数据库表模型,支持BigDecimal ,DateTime ,rust基本类型（int,float,uint,string,Vec,Array）
/// CRUDEnable 特性会自动识别 id为表的id类型(识别String)，自动识别结构体名称为蛇形命名的表名 biz_activity。没有id的表 请手动指定
#[crud_enable]
#[derive(Clone, Debug)]
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
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

// (可选) 手动实现，不使用上面的derive(CRUDEnable)和#[crud_enable],可重写table_name方法。手动实现能支持IDE智能提示
// impl CRUDEnable for BizActivity {
//     type IdType = String;
// }

//示例 mysql 链接地址
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  static ref RB:Rbatis=Rbatis::new();
}

//启动web服务，并且对表执行 count统计
#[async_std::main]
async fn main() {
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
    RB.link(MYSQL_URL).await.unwrap();
    let mut app = tide::new();
    app.at("/").get(|_: Request<()>| async move {
        let v = RB.list::<BizActivity>("").await;
        Ok(serde_json::json!(v).to_string())
    });
    let addr = "127.0.0.1:8000";
    println!("http server listen on http://{}",addr);
    app.listen(addr).await.unwrap();
}