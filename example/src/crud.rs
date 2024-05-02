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

//custom table name
//crud!(Activity {},"activity");
crud!(Activity {}); // impl_insert!($table {}) + impl_select!($table {}) + impl_update!($table {}) + impl_delete!($table {});

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
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
        version: Some(1),
        delete_flag: Some(1),
    };
    let tables = [table.clone(), {
        let mut t3 = table.clone();
        t3.id = "3".to_string().into();
        t3
    }];

    let data = Activity::insert(&rb, &table).await;
    println!("insert = {}", json!(data));

    let data = Activity::insert_batch(&rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));

    let data = Activity::update_by_column_batch(&rb, &tables, "id", 1).await;
    println!("update_by_column_batch = {}", json!(data));

    let data = Activity::update_by_column(&rb, &table, "id").await;
    println!("update_by_column = {}", json!(data));

    let data = Activity::delete_by_column(&rb, "id", "2").await;
    println!("delete_by_column = {}", json!(data));

    let data = Activity::select_in_column(&rb, "id", &["1", "2", "3"]).await;
    println!("select_in_column = {}", json!(data));

    let data = Activity::delete_in_column(&rb, "id", &["1", "2", "3"]).await;
    println!("delete_in_column = {}", json!(data));
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
