use rbatis::crud;
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;
use rbs::value;
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

// Generate CRUD methods including the new update_by_columns
crud!(Activity {});

// cargo run --bin crud_update_skip
#[tokio::main]
pub async fn main() -> Result<(), rbatis::Error> {
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

    // Create a sample activity record
    let activity = Activity {
        id: Some("123".into()),
        name: Some("Original Activity".into()),
        pc_link: Some("https://example.com/pc".into()),
        h5_link: Some("https://example.com/h5".into()),
        pc_banner_img: Some("pc_banner.jpg".into()),
        h5_banner_img: Some("h5_banner.jpg".into()),
        sort: Some("1".to_string()),
        status: Some(1),
        remark: Some("Original remark".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(0),
    };

    // Insert activity
    let insert_result = Activity::insert(&rb, &activity).await;
    println!("Insert result: {}", json!(insert_result));

    // Test regular update (should work)
    let regular_result = Activity::update_by_map(&rb, &activity, value! {"id": "123"}).await;
    println!("Regular update result: {}", json!(regular_result));

    // Update only name and status columns using set parameter
    let result = Activity::update_by_map(
        &rb,
        &activity,
        value! {"id": "123", "column": ["name", "status"]},
    )
    .await;
    println!("Update result (only name and status): {}", json!(result));
    Ok(())
}
