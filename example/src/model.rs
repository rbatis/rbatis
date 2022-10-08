use log::LevelFilter;
use rbatis::rbatis::Rbatis;
use rbatis::rbdc::datetime::FastDateTime;
use serde::{Deserialize, Serialize};

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
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();

    // // ------------sync tables------------
    // use rbatis::rbdc::db::Driver;
    // use rbatis::table_sync::{RbatisTableSync, SqliteTableSync};
    // let mut s = RbatisTableSync::new();
    // let driver = SqliteDriver {};
    // s.insert(driver.name().to_string(), Box::new(SqliteTableSync {}));
    // let raw = fast_log::LOGGER.get_level().clone();
    // fast_log::LOGGER.set_level(LevelFilter::Off);
    // s.sync(
    //     driver.name(),
    //     rb.acquire().await.unwrap(),
    //     &BizActivity {
    //         id: None,
    //         name: None,
    //         pc_link: None,
    //         h5_link: None,
    //         pc_banner_img: None,
    //         h5_banner_img: None,
    //         sort: None,
    //         status: None,
    //         remark: None,
    //         create_time: None,
    //         version: None,
    //         delete_flag: None,
    //     },
    // )
    // .await
    // .unwrap();
    // fast_log::LOGGER.set_level(raw);
    // // ------------sync tables end------------

    // ------------create tables way 2------------
    let sql = std::fs::read_to_string("example/table_sqlite.sql").unwrap();
    let raw = fast_log::LOGGER.get_level().clone();
    fast_log::LOGGER.set_level(LevelFilter::Off);
    let _ = rb.exec(&sql, vec![]).await;
    fast_log::LOGGER.set_level(raw);
    // ------------create tables way 2 end------------

    return rb;
}
