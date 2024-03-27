#[macro_use]
extern crate rbatis;

use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use serde_json::json;

/// table
#[derive(serde::Serialize, serde::Deserialize)]
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
crud!(Activity {});
impl_update!(Activity{update_by_name(name:&str) => "`where name = #{name}`"});

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
    }, "activity").await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);

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

    let data = Activity::update_by_column(&rb, &table, "id").await;
    println!("update_by_column = {}", json!(data));

    //not skip null value
    let data = Activity::update_by_column_skip(&rb, &table, "id",false).await;
    println!("update_by_column = {}", json!(data));

    let data = Activity::update_by_name(&rb, &table, "2").await;
    println!("update_by_name = {}", json!(data));


    let data = Activity::update_by_column_batch(
        &rb,
        &[
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
                version: Some(1),
                delete_flag: Some(1),
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
        ],
        "id",
        2,
    )
    .await;
    println!("update_by_column = {}", json!(data));
}
