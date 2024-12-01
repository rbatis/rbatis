use rbatis::dark_std::defer;
use rbatis::rbatis::RBatis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync;
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
    //rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;").unwrap();
    rb.init(SqliteDriver {}, &"sqlite://target/sqlite.db".to_string()).unwrap();

    // ------------choose column mapper------------
    let mapper = &table_sync::SqliteTableMapper {};
    //let mapper = &table_sync::PGTableMapper{} ;
    //let mapper = &table_sync::MysqlTableMapper{} ;
    //let mapper = &table_sync::MssqlTableMapper{} ;

    // let table = RBUser{};
    let table = to_value! {
        "id": "INTEGER",
        "name": "TEXT",
        "remark": "TEXT",
        "create_time": "TEXT",
        "version": "TEXT",
        "delete_flag": "INT8"
    };
    RBatis::sync(&rb.acquire().await.unwrap(), mapper, &table, "rb_user").await.unwrap();

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
