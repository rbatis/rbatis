
use log::LevelFilter;
use rbatis::{crud, RBatis};
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Base {
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    #[serde(flatten)]
    pub base: Base,
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}
crud!(Activity {});

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(||{log::logger().flush();});

    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // table sync done
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _=RBatis::sync(&rb.acquire().await.unwrap(), &SqliteTableMapper{}, &Activity{
        base: Base{ pc_banner_img: Some(String::new()), h5_banner_img: Some(String::new()) },
        id: Some(String::new()),
        name: Some(String::new()),
        pc_link: Some(String::new()),
        h5_link: Some(String::new()),
        sort: Some(String::new()),
        status: Some(0),
        remark: Some(String::new()),
        create_time: Some(DateTime::now()),
        version: Some(0),
        delete_flag: Some(0),
    }, "activity").await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);

    let datas = Activity::select_all(&rb.clone()).await.unwrap();
    println!("{}", json!(datas));
}
