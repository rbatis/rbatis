use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, RBatis};
use rbs::Value;
use serde_json::json;
use std::sync::Arc;
use rbatis::dark_std::sync::SyncVec;
use rbatis::rbdc::datetime::DateTime;
use rbatis::crud;

/// Mock intercept that just prints SQL
#[derive(Debug)]
pub struct MockIntercept;

#[async_trait]
impl Intercept for MockIntercept {
    async fn before(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        sql: &mut String,
        _args: &mut Vec<Value>,
        _result: ResultType<&mut Result<ExecResult, rbatis::Error>, &mut Result<Vec<Value>, rbatis::Error>>,
    ) -> Result<Option<bool>, rbatis::Error> {
        *sql = sql.replace("<my_table_name>", "activity");
        println!("MockIntercept: SQL = {}", sql);
        Ok(Some(true))
    }
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

//crud!(Activity {},"activity");
crud!(Activity {},"<my_table_name>");

#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console());
    let rb = RBatis::new();
    rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    
    let len = rb.intercepts.len();
    println!("len={}", len);
    
    // Create new intercept list and add our mock intercept
    let new_intercept = SyncVec::new();
    let intercept: Arc<dyn Intercept> = Arc::new(MockIntercept {});
    new_intercept.push(intercept);
    
    // Create connection and replace its intercepts
    let mut conn = rb.acquire().await.unwrap();
    conn.intercepts = Arc::new(new_intercept);
    println!("conn.intercepts.len={}", conn.intercepts.len());
    
    // Execute query to see the mock intercept in action
    let _ = conn.query("SELECT <my_table_name>", vec![]).await;
    let data = Activity::select_all(&conn).await.unwrap();
    println!("data={:?}", json!(data));
}