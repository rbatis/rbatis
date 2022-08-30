use rbatis::rbatis::Rbatis;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};
use rbdc_sqlite::driver::SqliteDriver;

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
    pub create_time: Option<FastDateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = Rbatis::new();
    rb.init(SqliteDriver {}, &format!("sqlite://target/sqlite.db"))
        .unwrap();
    let mut s = RbatisTableSync::new();
    s.insert("sqlite".to_string(), Box::new(SqliteTableSync {}));
    let t = BizActivity {
        id: Some("".to_string()),
        name: Some("".to_string()),
        pc_link: Some("".to_string()),
        h5_link: Some("".to_string()),
        pc_banner_img: Some("".to_string()),
        h5_banner_img: Some("".to_string()),
        sort: Some("".to_string()),
        status: Some(1),
        remark: Some("".to_string()),
        create_time: Some(FastDateTime::utc()),
        version: Some(1),
        delete_flag: Some(1),
    };
    s.sync("sqlite", rb.acquire().await.unwrap(), t)
        .await
        .unwrap();
}
