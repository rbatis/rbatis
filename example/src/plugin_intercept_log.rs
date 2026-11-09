use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::intercept_log::LogInterceptor;
use rbatis::{crud, Error, RBatis};
use rbs::value;
use std::time::Duration;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
}
crud!(Activity {});

/// control log level or close log
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

    //default rb.intercepts[0] = LogInterceptor{};
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db")?;

    println!("-----------------log level = info--------------------------------------");
    rb.get_intercept::<LogInterceptor>()
        .unwrap()
        .set_level_filter(LevelFilter::Info);
    _ = Activity::select_by_map(&rb, value! {"id":"2"}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("-----------------------------------------------------------------------");

    println!("------------------log level = off--------------------------------------");
    rb.get_intercept::<LogInterceptor>()
        .unwrap()
        .set_level_filter(LevelFilter::Off);
    _ = Activity::select_by_map(&rb, value! {"id":"2"}).await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("-----------------------------------------------------------------------");

    log::logger().flush();
    Ok(())
}
