use std::sync::{Arc, OnceLock};
use fast_log::Config;
use log::LevelFilter;
use rbatis::intercept::SqlIntercept;
use rbatis::intercept_log::LogInterceptor;
use rbatis::{crud, RBatis};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>
}
crud!(BizActivity {});

/// RBatis log also is a SqlIntercept
pub static RB_LOG: OnceLock<Arc<LogInterceptor>> = OnceLock::new();

#[tokio::main]
pub async fn main() {
    let logger=fast_log::init(Config::new().console()).unwrap();

    let rb = RBatis::new();
    rb.intercepts.clear();

    let l = Arc::new(LogInterceptor::new(LevelFilter::Info));
    rb.intercepts.push(l.clone() as Arc<dyn SqlIntercept>);
    //store to static
    _ = RB_LOG.set(l);

    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    ).unwrap();

    //default log level = info
    println!("log level = info");
    _ = BizActivity::select_all(&mut rb.clone()).await;
    logger.wait();
    println!("-------done-------");
    //set log level off (this can call set_level_filter any where)
    RB_LOG.get().unwrap().set_level_filter(LevelFilter::Off);
    println!("log level = off");
    _ = BizActivity::select_all(&mut rb.clone()).await;
    logger.wait();
    println!("-------done-------");
    //set log level trace (this can call set_level_filter any where)
    println!("log level = trace");
    RB_LOG.get().unwrap().set_level_filter(LevelFilter::Trace);
    _ = BizActivity::select_all(&mut rb.clone()).await;
    logger.wait();
    println!("-------done-------");
}