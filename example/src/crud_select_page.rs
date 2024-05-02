#[macro_use]
extern crate rbatis;

use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
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

impl_select_page!(Activity{select_page() =>"
     if !sql.contains('count(1)'):
       `order by create_time desc`"});
impl_select_page!(Activity{select_page_by_name(name:&str) =>"
     if name != null && name != '':
       `where name != #{name}`
     if name == '':
       `where name != ''`"});

impl_select_page!(Activity{select_page_by_limit(name:&str,limit_sql:&str) => "`where name != #{name}`"});

use rbatis::rbatis_codegen::IntoSql;
use rbatis::table_sync::SqliteTableMapper;

pysql_select_page!(select_page_data(name: &str) -> Activity =>
r#"`select * from activity where delete_flag = 0`
                  if name != '':
                    ` and name=#{name}`
                  if !ids.is_empty():
                    ` and id in `
                    ${ids.sql()}"#);

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

    let data = Activity::select_page(&rb, &PageRequest::new(1, 10)).await;
    println!("select_page = {}", json!(data));

    let data = Activity::select_page_by_name(&rb, &PageRequest::new(1, 10), "").await;
    println!("select_page_by_name = {}", json!(data));

    let data =
        Activity::select_page_by_limit(&rb, &PageRequest::new(1, 10), "2", " limit 0,10 ").await;
    println!("select_page_by_limit = {}", json!(data));

    let data = select_page_data(&rb, &PageRequest::new(1, 10), "2").await;
    println!("select_page_data = {}", json!(data));
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
