
use rbatis::executor::Executor;
use rbatis::intercept::{Intercept, ResultType};
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::{async_trait, crud, Error, RBatis};
use rbs::Value;
use serde_json::json;
use std::sync::Arc;
use rbatis::dark_std::defer;

/// Postgres insert sql returning id Intercept
#[derive(Debug)]
pub struct ReturningIdPlugin {}

#[async_trait]
impl Intercept for ReturningIdPlugin {
    async fn before(
        &self,
        _task_id: i64,
        rb: &dyn Executor,
        sql: &mut String,
        args: &mut Vec<Value>,
        result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Vec<Value>, Error>>,
    ) -> Result<bool, Error> {
        if sql.contains("insert into") {
            let new_sql = format!("{} {}", sql, "returning id");
            let new_args = args.clone();
            match result {
                ResultType::Exec(exec_r) => {
                    let id = rb.query(&new_sql, new_args).await?;
                    let id: String = rbatis::decode(id)?;
                    let mut exec = ExecResult::default();
                    exec.last_insert_id = id.into();
                    *exec_r = Ok(exec);
                    Ok(false)
                }
                ResultType::Query(_) => Ok(true),
            }
        } else {
            Ok(true)
        }
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

//docker run -d --name postgres  -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(log::LevelFilter::Debug));
    defer!(|| log::logger().flush());
    let rb = RBatis::new();
    rb.init(
        rbdc_pg::driver::PgDriver {},
        "postgres://postgres:123456@localhost:5432/postgres",
    )
    .unwrap();
    //insert to log intercept before
    rb.intercepts.insert(0, Arc::new(ReturningIdPlugin {}));
    let table = Activity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("2".to_string()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let data = Activity::insert(&rb, &table).await;
    println!("insert = {}", json!(data));
}
