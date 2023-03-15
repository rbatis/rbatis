use log::LevelFilter;
use rbatis::rbatis::Rbatis;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use rbatis::dark_std::defer;

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
    pub create_time: Option<DateTime>,
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
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();

    // ------------create tables way 2------------
    let sql = std::fs::read_to_string("example/table_sqlite.sql").unwrap();
    let raw = fast_log::LOGGER.get_level().clone();
    fast_log::LOGGER.set_level(LevelFilter::Off);
    defer!(||{
         fast_log::LOGGER.set_level(raw);
    });
    let _ = rb.exec(&sql, vec![]).await;
    // ------------create tables way 2 end------------
    return rb;
}
