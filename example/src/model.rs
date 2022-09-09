use log::LevelFilter;
use rbatis::rbatis::Rbatis;
use rbatis::rbdc::datetime::FastDateTime;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

/// example table
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
pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(SqliteDriver {}, "sqlite://target/sqlite.db", ).unwrap();

    // ------------create tables------------
    let mut f = File::open("example/table_sqlite.sql").unwrap();
    let mut sql = String::new();
    f.read_to_string(&mut sql).unwrap();
    fast_log::LOGGER.set_level(LevelFilter::Off);
    let _ = rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(LevelFilter::Info);
    // ------------create tables end------------

    return rb;
}