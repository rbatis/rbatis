use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{Error, RBatis};
use rbs::value;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RBUser {
    pub id: i32,
    pub name: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db")?;

    // let table = RBUser{};
    let table = value! {
        "id": "INTEGER",
        "name": "TEXT",
        "remark": "TEXT",
        "create_time": "TEXT",
        "version": "TEXT",
        "delete_flag": "INT8"
    };
    RBatis::sync(&rb.acquire().await?, &rb, &table, "rb_user").await?;

    //sync table struct
    let table = RBUser {
        id: 0,
        //name: Some("TEXT".to_string()),// Custom String Database Type
        name: Some("".to_string()),
        //remark: Some("TEXT".to_string()),// Custom String Database Type
        remark: Some("".to_string()),
        create_time: Some(DateTime::utc()),
        version: Some(1),
        delete_flag: Some(1),
    };
    RBatis::sync(&rb.acquire().await?, &rb, &table, "rb_user").await?;
    Ok(())
}
