use log::{LevelFilter};
use rbatis::intercept_log::LogInterceptor;
use rbatis::{crud, RBatis};
use std::time::Duration;
use rbatis::dark_std::defer;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
}
crud!(Activity {});

/// control log level or close log
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(||{log::logger().flush();});

    //default rb.intercepts[0] = LogInterceptor{};
    let rb = RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();


    println!("-----------------log level = info--------------------------------------");
    rb.get_intercept::<LogInterceptor>().unwrap().set_level_filter(LevelFilter::Info);
    _ = Activity::select_by_column(&rb.clone(),"id","2").await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("-----------------------------------------------------------------------");

    println!("------------------log level = off--------------------------------------");
    rb.get_intercept::<LogInterceptor>().unwrap().set_level_filter(LevelFilter::Off);
    _ = Activity::select_by_column(&rb.clone(),"id","2").await;
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("-----------------------------------------------------------------------");

    log::logger().flush();
}
