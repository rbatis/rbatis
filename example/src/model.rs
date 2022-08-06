use log::LevelFilter;
use rbatis::rbatis::Rbatis;
use rbdc::datetime::FastDateTime;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Read;

/// this is table model(see ../database.sql)
#[derive(Clone, Debug, Serialize, Deserialize)]
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

/// make a sqlite-rbatis
pub async fn init_sqlite() -> Rbatis {
    init_sqlite_path("").await
}

/// make a sqlite-rbatis
pub async fn init_sqlite_path(path: &str) -> Rbatis {
    //first init log carte
    fast_log::init(fast_log::Config::new().console());

    let rb = Rbatis::new();
    // rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
    // rb.link("postgres://postgres:123456@localhost:5432/postgres").await.unwrap();
    // rb.link("mssql://SA:TestPass!123456@localhost:1433/test").await.unwrap();
    rb.link(
        SqliteDriver {},
        &format!("sqlite://{}target/sqlite.db", path),
    )
    .await
    .unwrap();

    // ------------carte table------------
    let mut f = File::open(format!("{}example/table_sqlite.sql", path)).unwrap();
    let mut sql = String::new();
    f.read_to_string(&mut sql).unwrap();
    fast_log::LOGGER.set_level(LevelFilter::Off);
    rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(LevelFilter::Info);
    // ------------carte table end------------

    return rb;
}
