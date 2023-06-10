use rbatis::rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RBUser {
    pub id: i32,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    let rb = RBatis::new();
    rb.init(SqliteDriver {}, &format!("sqlite://target/sqlite.db"))
        .unwrap();
    let mut s = SqliteTableSync::default();
    s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
    s.sync(
        rb.acquire().await.unwrap(),
        to_value!(RBUser {
            id: 0,
            name: Some("".to_string()),
            remark: Some("".to_string()),
            create_time: Some(DateTime::utc()),
            version: Some(1),
            delete_flag: Some(1),
        }),
        "rb_user",
    )
    .await
    .unwrap();
}
