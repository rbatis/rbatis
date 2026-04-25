use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{crud, Error, RBatis};
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

crud!(Activity {});

#[tokio::main]
pub async fn main() -> Result<(), Error> {
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
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )?;

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

    let total = 100;
    let mut tables = Vec::with_capacity(total);
    for i in 0..total {
        let mut table = table.clone();
        table.id = Some(i.to_string());
        table.name = Some(i.to_string());
        table.pc_link = Some(i.to_string());
        table.h5_link = Some(i.to_string());
        table.pc_banner_img = None;
        table.h5_banner_img = None;
        table.sort = Some(i.to_string());
        table.status = Some(1);
        table.remark = Some(i.to_string());
        table.create_time = Some(DateTime::now());
        table.version = Some(1);
        tables.push(table);
    }
    let data = Activity::insert_batch(&rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));
    Ok(())
}
