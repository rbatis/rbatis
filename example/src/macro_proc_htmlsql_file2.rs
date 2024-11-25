use log::LevelFilter;
use rbatis::dark_std::defer;
use serde_json::json;

use rbatis::executor::Executor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::{html_sql, RBatis};
use rbatis::rbdc::db::ExecResult;

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

use rbatis::rbatis_codegen::IntoSql;
#[html_sql("example.html")]
pub async fn insert(
    rb: &dyn Executor,
    arg: &Activity,
) -> rbatis::Result<ExecResult> {
    impled!()
}

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
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // table sync done
    fast_log::logger().set_level(LevelFilter::Off);
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
    fast_log::logger().set_level(LevelFilter::Debug);

    let a = insert(&rb, &Activity{
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
    })
        .await
        .unwrap();
    println!("{}", json!(a));
}
