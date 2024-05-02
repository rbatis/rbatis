#[macro_use]
extern crate rbatis;

use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::RBatis;
use serde_json::json;

/// table
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Activity {
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

impl_insert!(Activity {});

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();
    // table sync done
    sync_table(&rb).await;

    let table = Activity {
        id: Some("1".into()),
        name: Some("1".into()),
        pc_link: Some("1".into()),
        h5_link: Some("1".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("1".to_string()),
        status: Some(1),
        remark: Some("1".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };

    let data = Activity::insert(&rb, &table).await;
    println!("insert = {}", json!(data));

    let tables = vec![
        Activity {
            id: Some("2".into()),
            name: Some("2".into()),
            pc_link: Some("2".into()),
            h5_link: Some("2".into()),
            pc_banner_img: None,
            h5_banner_img: None,
            sort: Some("2".to_string()),
            status: Some(2),
            remark: Some("2".into()),
            create_time: Some(DateTime::now()),
            version: Some(2),
            delete_flag: Some(2),
        },
        Activity {
            id: Some("3".into()),
            name: Some("3".into()),
            pc_link: Some("3".into()),
            h5_link: Some("3".into()),
            pc_banner_img: None,
            h5_banner_img: None,
            sort: Some("3".to_string()),
            status: Some(3),
            remark: Some("3".into()),
            create_time: Some(DateTime::now()),
            version: Some(3),
            delete_flag: Some(3),
        },
    ];
    let data = Activity::insert_batch(&rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));
}

async fn sync_table(rb: &RBatis) {
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _ = RBatis::sync(
        &rb.acquire().await.unwrap(),
        &SqliteTableMapper {},
        &Activity {
            id: Some(String::new()),
            name: Some(String::new()),
            pc_link: Some(String::new()),
            h5_link: Some(String::new()),
            pc_banner_img: Some(String::new()),
            h5_banner_img: Some(String::new()),
            sort: Some(String::new()),
            status: Some(0),
            remark: Some(String::new()),
            create_time: Some(DateTime::now()),
            version: Some(0),
            delete_flag: Some(0),
        },
        "activity",
    )
    .await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);
}
