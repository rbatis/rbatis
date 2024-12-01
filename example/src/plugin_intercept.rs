use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::{async_trait, crud, Error, RBatis};
use rbs::Value;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use rbatis::dark_std::sync::SyncHashMap;

/// Logic delete： The deletion statement changes to the modification of flag, and the query statement filters flag with additional conditions
#[derive(Debug)]
pub struct CountTimeIntercept {
    map: SyncHashMap<i64, Instant>,
}

#[async_trait]
impl Intercept for CountTimeIntercept {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<Option<bool>, Error> {
        self.map.insert(_task_id, Instant::now());
        Ok(Some(true))
    }
    async fn after(&self, task_id: i64, _rb: &dyn Executor, _sql: &mut String, _args: &mut Vec<Value>, _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>) -> Result<Option<bool>, Error> {
        if let Some(v) = self.map.remove(&task_id) {
            println!("[{}] use time={:?}", task_id, v.elapsed());
        }
        Ok(Some(true))
    }
}

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

crud!(Activity {});

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
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;").unwrap();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // table sync done
    fast_log::logger().set_level(LevelFilter::Off);
    _ = RBatis::sync(
        &rb.acquire().await.unwrap(),
        &SqliteTableMapper {},
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
    fast_log::logger().set_level(LevelFilter::Debug);

    rb.intercepts.push(Arc::new(CountTimeIntercept { map: Default::default() }));

    //get intercept
    let intercept = rb.get_intercept::<CountTimeIntercept>().unwrap();
    println!("intercept name = {}", intercept.name());
    //query
    let r = Activity::delete_by_column(&rb, "id", "1").await;
    println!("{}", json!(r));
    let record = Activity::select_by_column(&rb, "id", "1").await;
    println!("{}", json!(record));
}
