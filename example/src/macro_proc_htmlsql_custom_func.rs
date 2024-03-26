#[macro_use]
extern crate rbatis;

use log::LevelFilter;
use rbatis::dark_std::defer;
use serde_json::json;

use rbatis::executor::Executor;
use rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use rbs::Value;

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

pub trait IsTest {
    fn is_test(&self) -> bool;
}

impl IsTest for rbs::Value {
    fn is_test(&self) -> bool {
        match self {
            Value::String(v) => v.eq("test"),
            _ => false,
        }
    }
}

/// you can see custom fn on xml:
/// ```xml
/// <if test="name.is_test()">
///    ....
///  </if>
/// ```
#[html_sql(
    r#"
    <select id="custom_func">
        `select * from activity`
        <where>
            <if test="name.is_test()">
                `and name like #{name}`
            </if>
        </where>
    </select>"#
)]
async fn custom_func(rb: &dyn Executor, name: &str) -> rbatis::Result<Vec<Activity>> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(||{log::logger().flush();});

    //use static ref
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

    let a = custom_func(&rb, "test").await.unwrap();
    println!("{}", json!(a));
}
