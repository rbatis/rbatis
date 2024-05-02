use rbatis::dark_std::defer;
use rbatis::rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync;
use rbatis::table_sync::ColumMapper;

use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;

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
    // ------------choose driver------------
    //rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    //rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    //rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(SqliteDriver {}, &"sqlite://target/sqlite.db".to_string())
        .unwrap();
    // ------------choose column mapper------------
    let mapper = &table_sync::SqliteTableMapper {} as &dyn ColumMapper;
    //let mapper = &table_sync::PGTableMapper{} as &dyn ColumMapper;
    //let mapper = &table_sync::MysqlTableMapper{} as &dyn ColumMapper;
    //let mapper = &table_sync::MssqlTableMapper{} as &dyn ColumMapper;

    // sync table to_value
    RBatis::sync(&rb.acquire().await.unwrap(), mapper, &to_value! {"id": "INTEGER","name": "TEXT","remark": "TEXT","create_time": "TEXT","version": "TEXT","delete_flag": "INT8"}, "rb_user").await.unwrap();

    //sync table struct
    RBatis::sync(
        &rb.acquire().await.unwrap(),
        mapper,
        &RBUser {
            id: 0,
            //name: Some("TEXT".to_string()),// Custom String Database Type
            name: Some("".to_string()),
            //remark: Some("TEXT".to_string()),// Custom String Database Type
            remark: Some("".to_string()),
            create_time: Some(DateTime::utc()),
            version: Some(1),
            delete_flag: Some(1),
        },
        "rb_user",
    )
    .await
    .unwrap();
}
