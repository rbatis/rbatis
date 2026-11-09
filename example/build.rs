use log::LevelFilter;
use rbatis::rbdc::datetime::DateTime;
use rbatis::RBatis;

/// this just only to create database table show example, you don't need this code
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
            // the src/ examples open `sqlite://target/sqlite.db` relative to the directory cargo
            // was invoked from:
            //   `cargo run -p example --bin X` from the repo root  -> ../target/sqlite.db
            //   `cargo run --bin X`           from the example/ dir ->  target/sqlite.db
            // create the table in both so that either invocation works
            for url in ["sqlite://../target/sqlite.db", "sqlite://target/sqlite.db"] {
                _ = std::fs::create_dir_all(url.trim_start_matches("sqlite://"));
                let rb = RBatis::new();
                if rb.init(rbdc_sqlite::SqliteDriver {}, url).is_err() {
                    continue;
                }
                let conn = match rb.acquire().await {
                    Ok(conn) => conn,
                    Err(_) => continue,
                };
                _ = RBatis::sync(
                    &conn,
                    &rb,
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
            }
            log::logger().flush();
        });
}

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
